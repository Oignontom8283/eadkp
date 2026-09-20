<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

type Point = { x: number; y: number }
type Selection = {
  id: number
  type: 'point' | 'rect'
  text: string
}

const canvasWidth = 320
const canvasHeight = 240
const canvas = ref<HTMLCanvasElement | null>(null)
const history = ref<Selection[]>([])
const pointer = ref<Point | null>(null)
const dragStart = ref<Point | null>(null)
const dragCurrent = ref<Point | null>(null)
const isDragging = ref(false)
const copiedId = ref<number | null>(null)
const copyErrorId = ref<number | null>(null)
const activePointerId = ref<number | null>(null)
let nextSelectionId = 0
let themeObserver: MutationObserver | null = null

const selection = computed(() => {
  if (!dragStart.value || !dragCurrent.value) return null

  return {
    left: Math.min(dragStart.value.x, dragCurrent.value.x),
    top: Math.min(dragStart.value.y, dragCurrent.value.y),
    width: Math.abs(dragCurrent.value.x - dragStart.value.x),
    height: Math.abs(dragCurrent.value.y - dragStart.value.y),
  }
})

const coordinateLabel = computed(() => {
  if (!pointer.value) return 'x: —  y: —'
  if (!isDragging.value || !dragStart.value) {
    return `x: ${pointer.value.x}  y: ${pointer.value.y}`
  }
  return `(${dragStart.value.x}, ${dragStart.value.y})  →  (${pointer.value.x}, ${pointer.value.y})`
})

function drawGrid() {
  const context = canvas.value?.getContext('2d')
  if (!context) return

  const styles = getComputedStyle(canvas.value!)
  const background = styles.getPropertyValue('--vp-c-bg-alt').trim()
  const minorLine = styles.getPropertyValue('--vp-c-divider').trim()
  const majorLine = styles.getPropertyValue('--vp-c-text-3').trim()
  const label = styles.getPropertyValue('--vp-c-text-2').trim()

  context.fillStyle = background
  context.fillRect(0, 0, canvasWidth, canvasHeight)
  context.lineWidth = 1

  for (let x = 20; x < canvasWidth; x += 20) {
    context.strokeStyle = x % 100 === 0 ? majorLine : minorLine
    context.beginPath()
    context.moveTo(x + 0.5, 0)
    context.lineTo(x + 0.5, canvasHeight)
    context.stroke()
  }

  for (let y = 20; y < canvasHeight; y += 20) {
    context.strokeStyle = y % 100 === 0 ? majorLine : minorLine
    context.beginPath()
    context.moveTo(0, y + 0.5)
    context.lineTo(canvasWidth, y + 0.5)
    context.stroke()
  }

  context.fillStyle = label
  context.font = '8px monospace'
  for (let x = 100; x < canvasWidth; x += 100) context.fillText(String(x), x + 2, 9)
  for (let y = 100; y < canvasHeight; y += 100) context.fillText(String(y), 2, y - 2)
}

function getPoint(event: PointerEvent): Point {
  const element = canvas.value!
  const bounds = element.getBoundingClientRect()
  const scaleX = canvasWidth / bounds.width
  const scaleY = canvasHeight / bounds.height

  return {
    x: Math.round(Math.min(Math.max((event.clientX - bounds.left) * scaleX, 0), canvasWidth - 1)),
    y: Math.round(Math.min(Math.max((event.clientY - bounds.top) * scaleY, 0), canvasHeight - 1)),
  }
}

function startSelection(event: PointerEvent) {
  if (event.pointerType === 'mouse' && event.button !== 0) return
  event.preventDefault()
  canvas.value?.setPointerCapture(event.pointerId)
  activePointerId.value = event.pointerId
  const point = getPoint(event)
  pointer.value = point
  dragStart.value = point
  dragCurrent.value = point
  isDragging.value = true
}

function updateSelection(event: PointerEvent) {
  if (!isDragging.value) {
    pointer.value = getPoint(event)
    return
  }
  event.preventDefault()
  dragCurrent.value = getPoint(event)
  pointer.value = dragCurrent.value
}

function finishSelection(event: PointerEvent) {
  if (!isDragging.value || !dragStart.value || event.pointerId !== activePointerId.value) return
  event.preventDefault()
  const end = getPoint(event)
  const start = dragStart.value
  const width = Math.abs(end.x - start.x)
  const height = Math.abs(end.y - start.y)
  const id = nextSelectionId++

  history.value.unshift(width < 4 && height < 4
    ? { id, type: 'point', text: `Point { x: ${start.x}, y: ${start.y} }` }
    : {
        id,
        type: 'rect',
        text: `Rect { x: ${Math.min(start.x, end.x)}, y: ${Math.min(start.y, end.y)}, width: ${width}, height: ${height} }`,
      })

  pointer.value = end
  dragStart.value = null
  dragCurrent.value = null
  isDragging.value = false
  activePointerId.value = null
}

function cancelSelection(event: PointerEvent) {
  if (event.pointerId !== activePointerId.value) return
  dragStart.value = null
  dragCurrent.value = null
  isDragging.value = false
  activePointerId.value = null
  pointer.value = null
}

async function copySelection(item: Selection) {
  try {
    await navigator.clipboard.writeText(item.text)
    copiedId.value = item.id
    copyErrorId.value = null
    window.setTimeout(() => {
      if (copiedId.value === item.id) copiedId.value = null
    }, 1500)
  } catch {
    copiedId.value = null
    copyErrorId.value = item.id
  }
}

function clearHistory() {
  history.value = []
  copiedId.value = null
  copyErrorId.value = null
}

function resetPointer() {
  if (!isDragging.value) pointer.value = null
}

onMounted(() => {
  drawGrid()
  themeObserver = new MutationObserver(drawGrid)
  themeObserver.observe(document.documentElement, {
    attributeFilter: ['class'],
    attributes: true,
  })
})

onBeforeUnmount(() => {
  themeObserver?.disconnect()
})
</script>

<template>
  <section class="dimensions-picker" aria-label="Coordinate picker">
    <output class="coordinate-display" aria-live="polite">{{ coordinateLabel }}</output>

    <div class="picker-layout">
      <div class="canvas-wrap">
        <canvas
          ref="canvas"
          :width="canvasWidth"
          :height="canvasHeight"
          aria-label="Coordinate selection area"
          @pointerdown="startSelection"
          @pointermove="updateSelection"
          @pointerup="finishSelection"
          @pointercancel="cancelSelection"
          @pointerleave="resetPointer"
        />
        <div
          v-if="selection"
          class="selection-overlay"
          :style="{
            left: `${selection.left / canvasWidth * 100}%`,
            top: `${selection.top / canvasHeight * 100}%`,
            width: `${selection.width / canvasWidth * 100}%`,
            height: `${selection.height / canvasHeight * 100}%`,
          }"
        />
      </div>

      <aside class="history-panel">
        <header class="history-header">
          <span>History</span>
          <button type="button" class="clear-button" :disabled="!history.length" @click="clearHistory">
            Clear
          </button>
        </header>
        <div v-if="!history.length" class="empty-history">Click or drag on the grid.</div>
        <div v-else class="history-list">
          <button
            v-for="item in history"
            :key="item.id"
            type="button"
            class="history-entry"
            :class="item.type"
            @click="copySelection(item)"
          >
            <span class="entry-badge">{{ item.type === 'point' ? 'Point' : 'Rectangle' }}</span>
            <code>{{ item.text }}</code>
            <span class="entry-hint">
              {{ copiedId === item.id ? 'Copied!' : copyErrorId === item.id ? 'Copy unavailable' : 'Click to copy' }}
            </span>
          </button>
        </div>
      </aside>
    </div>
  </section>
</template>

<style scoped>
.dimensions-picker {
  --picker-accent: var(--vp-c-brand-1);
  --picker-accent-soft: var(--vp-c-brand-soft);
  --picker-panel: var(--vp-c-bg-soft);
  --picker-border: var(--vp-c-divider);
  --picker-muted: var(--vp-c-text-2);
  --picker-grid-background: var(--vp-c-bg-alt);
  --picker-grid-minor: var(--vp-c-divider);
  --picker-grid-major: var(--vp-c-text-3);
  --picker-grid-label: var(--vp-c-text-2);
  margin: 2rem 0;
  color: var(--vp-c-text-1);
}

.coordinate-display {
  display: block;
  width: 100%;
  margin-bottom: 0.75rem;
  padding: 0.45rem 0.75rem;
  border: 1px solid var(--picker-border);
  border-radius: 6px;
  background: var(--picker-panel);
  color: var(--picker-accent);
  font: 0.8rem var(--vp-font-family-mono);
  text-align: center;
}

.picker-layout {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
}

.canvas-wrap {
  position: relative;
  flex: 1 1 320px;
  min-width: 0;
  max-width: 320px;
  aspect-ratio: 4 / 3;
}

canvas {
  display: block;
  width: 100%;
  height: 100%;
  background: var(--vp-c-bg-alt);
  border: 1px solid var(--picker-border);
  border-radius: 4px;
  cursor: crosshair;
  touch-action: none;
}

.selection-overlay {
  position: absolute;
  border: 1px dashed var(--picker-accent);
  background: var(--picker-accent-soft);
  pointer-events: none;
}

.history-panel {
  display: flex;
  flex: 1 1 240px;
  flex-direction: column;
  min-width: 0;
  max-width: 280px;
  height: 240px;
  overflow: hidden;
  border: 1px solid var(--picker-border);
  border-radius: 6px;
  background: var(--picker-panel);
}

.history-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.45rem 0.7rem;
  border-bottom: 1px solid var(--picker-border);
  color: var(--picker-muted);
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.clear-button {
  padding: 0.2rem 0.45rem;
  border: 1px solid var(--picker-border);
  border-radius: 4px;
  background: transparent;
  color: var(--picker-muted);
  cursor: pointer;
  font: inherit;
  font-size: 0.7rem;
}

.clear-button:hover:not(:disabled) {
  border-color: var(--vp-c-danger-1);
  color: var(--vp-c-danger-1);
}

.clear-button:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

.empty-history {
  padding: 1rem 0.75rem;
  color: var(--picker-muted);
  font-size: 0.75rem;
  line-height: 1.5;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
  overflow-y: auto;
  padding: 0.4rem;
}

.history-entry {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.15rem;
  padding: 0.4rem 0.55rem;
  border: 1px solid var(--picker-border);
  border-left: 3px solid var(--picker-accent);
  border-radius: 4px;
  background: var(--vp-c-bg);
  color: var(--vp-c-text-1);
  cursor: pointer;
  font: inherit;
  text-align: left;
}

.history-entry.rect {
  border-left-color: var(--vp-c-green-1);
}

.history-entry:hover,
.history-entry:focus-visible {
  border-color: var(--picker-accent);
  outline: none;
}

.entry-badge,
.entry-hint {
  color: var(--picker-muted);
  font-size: 0.65rem;
}

.history-entry code {
  overflow-wrap: anywhere;
  color: inherit;
  font-size: 0.7rem;
}

.entry-hint {
  color: var(--vp-c-green-1);
}

@media (max-width: 640px) {
  .picker-layout {
    flex-direction: column;
  }

  .canvas-wrap,
  .history-panel {
    width: 100%;
    max-width: none;
  }

  .history-panel {
    height: auto;
    max-height: 240px;
  }
}
</style>
