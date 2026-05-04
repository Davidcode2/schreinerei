import { useEffect, useState } from 'react'
import { cn } from '@/lib/utils'
import { useSwipeGesture } from '@/hooks/useSwipeGesture'

interface StepContainerProps {
  currentStep: number
  onStepChange: (step: number) => void
  totalSteps: number
  children: React.ReactNode
  className?: string
}

export function StepContainer({
  currentStep,
  onStepChange,
  totalSteps,
  children,
  className,
}: StepContainerProps) {
  const [isAnimating, setIsAnimating] = useState(false)
  const [direction, setDirection] = useState<'left' | 'right'>('right')

  const handleSwipeLeft = () => {
    if (currentStep < totalSteps) {
      setDirection('left')
      setIsAnimating(true)
      onStepChange(currentStep + 1)
    }
  }

  const handleSwipeRight = () => {
    if (currentStep > 1) {
      setDirection('right')
      setIsAnimating(true)
      onStepChange(currentStep - 1)
    }
  }

  const swipeHandlers = useSwipeGesture({
    onSwipeLeft: handleSwipeLeft,
    onSwipeRight: handleSwipeRight,
    threshold: 50,
  })

  useEffect(() => {
    if (isAnimating) {
      const timer = setTimeout(() => setIsAnimating(false), 200)
      return () => clearTimeout(timer)
    }
  }, [isAnimating])

  return (
    <div
      {...swipeHandlers}
      className={cn(
        'transition-all duration-200 ease-out',
        isAnimating && direction === 'left' && 'animate-step-enter-left',
        isAnimating && direction === 'right' && 'animate-step-enter-right',
        className
      )}
    >
      {children}
    </div>
  )
}
