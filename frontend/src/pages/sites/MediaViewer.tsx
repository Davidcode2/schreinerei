import type { MediaViewerTarget } from "./mediaViewerRoute"

interface MediaViewerProps {
  open: boolean
  target: MediaViewerTarget | null
  onClose: () => void
}

export function MediaViewer(_: MediaViewerProps) {
  return null
}
