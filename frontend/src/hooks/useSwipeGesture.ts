import { useState, useCallback } from 'react'

interface SwipeGestureOptions {
  onSwipeLeft?: () => void
  onSwipeRight?: () => void
  threshold?: number
}

interface SwipeGestureHandlers {
  onTouchStart: (e: React.TouchEvent) => void
  onTouchEnd: (e: React.TouchEvent) => void
}

export function useSwipeGesture(options: SwipeGestureOptions): SwipeGestureHandlers {
  const { onSwipeLeft, onSwipeRight, threshold = 50 } = options
  const [touchStart, setTouchStart] = useState<number | null>(null)

  const onTouchStart = useCallback((e: React.TouchEvent) => {
    const touch = e.touches[0]
    if (!touch) return

    setTouchStart(touch.clientX)
  }, [])

  const onTouchEnd = useCallback(
    (e: React.TouchEvent) => {
      if (touchStart === null) return

      const touch = e.changedTouches[0]
      if (!touch) {
        setTouchStart(null)
        return
      }

      const touchEnd = touch.clientX
      const diff = touchStart - touchEnd

      if (Math.abs(diff) > threshold) {
        if (diff > 0) {
          onSwipeLeft?.()
        } else {
          onSwipeRight?.()
        }
      }

      setTouchStart(null)
    },
    [touchStart, threshold, onSwipeLeft, onSwipeRight]
  )

  return { onTouchStart, onTouchEnd }
}
