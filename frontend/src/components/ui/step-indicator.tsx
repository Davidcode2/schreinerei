import { cn } from '@/lib/utils'

interface StepIndicatorProps {
  currentStep: number
  totalSteps: number
  onStepClick?: (step: number) => void
  className?: string
}

export function StepIndicator({
  currentStep,
  totalSteps,
  onStepClick,
  className,
}: StepIndicatorProps) {
  return (
    <div
      className={cn('flex items-center justify-center gap-1.5 py-3', className)}
      role="tablist"
      aria-label="Schritte"
    >
      {Array.from({ length: totalSteps }, (_, index) => {
        const stepNumber = index + 1
        const isActive = stepNumber === currentStep
        const isCompleted = stepNumber < currentStep

        return (
          <button
            key={stepNumber}
            type="button"
            role="tab"
            aria-selected={isActive}
            aria-label={`Schritt ${stepNumber} von ${totalSteps}`}
            onClick={() => onStepClick?.(stepNumber)}
            className="flex h-11 w-11 items-center justify-center disabled:pointer-events-none"
            disabled={!onStepClick}
          >
            <span
              className={cn(
                'h-2 w-2 rounded-full transition-all',
                isActive || isCompleted
                  ? 'bg-primary'
                  : 'bg-muted-foreground/30'
              )}
            />
          </button>
        )
      })}
    </div>
  )
}
