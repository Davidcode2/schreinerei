import { Clock } from "lucide-react";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import {
	AlertDialog,
	AlertDialogAction,
	AlertDialogCancel,
	AlertDialogContent,
	AlertDialogDescription,
	AlertDialogFooter,
	AlertDialogHeader,
	AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
	useCreateTimeEntry,
	useDeleteTimeEntry,
	usePreferences,
	useSites,
	useUpdateTimeEntry,
} from "@/lib/api/hooks";
import { cn } from "@/lib/utils";
import type { TimeEntry, WorkType } from "@/types/sites";

interface TimeEntryDialogProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	siteId?: string;
	siteName?: string;
	mode?: "create" | "edit";
	initialData?: TimeEntry;
}

const workTypes: { value: WorkType; label: string }[] = [
	{ value: "site", label: "Baustelle" },
	{ value: "workshop", label: "Werkstatt" },
	{ value: "travel", label: "Fahrt" },
	{ value: "other", label: "Sonstiges" },
];

const quickHours = [0.5, 1, 2, 4, 8];

export function TimeEntryDialog({
	open,
	onOpenChange,
	siteId,
	siteName,
	mode = "create",
	initialData,
}: TimeEntryDialogProps) {
	const [workType, setWorkType] = useState<WorkType>(
		initialData?.work_type ?? "site",
	);
	const [hours, setHours] = useState(initialData?.hours ?? 0.5);
	const [workDate, setWorkDate] = useState(
		initialData?.work_date ?? new Date().toISOString().split("T")[0] ?? "",
	);
	const [notes, setNotes] = useState(initialData?.notes ?? "");
	const [selectedSiteId, setSelectedSiteId] = useState(
		initialData?.site_id ?? siteId ?? "",
	);
	const [errors, setErrors] = useState<Record<string, string>>({});
	const [touched, setTouched] = useState<Record<string, boolean>>({});
	const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

	const createMutation = useCreateTimeEntry();
	const updateMutation = useUpdateTimeEntry();
	const deleteMutation = useDeleteTimeEntry();
	const { data: preferences } = usePreferences();
	const { data: sites } = useSites();

	// Reset state when dialog opens with new data
	useEffect(() => {
		if (open) {
			if (mode === "edit" && initialData) {
				setWorkType(initialData.work_type);
				setHours(initialData.hours);
				setWorkDate(initialData.work_date);
				setNotes(initialData.notes ?? "");
				setSelectedSiteId(initialData.site_id ?? "");
			} else {
				setWorkType("site");
				setHours(0.5);
				setWorkDate(new Date().toISOString().split("T")[0] ?? "");
				setNotes("");
				setSelectedSiteId(siteId ?? preferences?.active_site_id ?? "");
			}
			setErrors({});
			setTouched({});
			setShowDeleteConfirm(false);
		}
	}, [open, mode, initialData, preferences?.active_site_id, siteId]);

	const validateHours = (value: number): string | null => {
		if (value <= 0) return "Stunden müssen größer als 0 sein";
		if (value > 24) return "Stunden dürfen 24 nicht überschreiten";
		return null;
	};

	const setHoursError = (error: string | null) => {
		if (error) {
			setErrors((prev) => ({ ...prev, hours: error }));
		} else {
			setErrors((prev) => {
				const { hours: _, ...rest } = prev;
				return rest;
			});
		}
	};

	const isFormValid = hours > 0 && hours <= 24 && workDate;

	const handleSubmit = async () => {
		try {
			if (mode === "edit" && initialData) {
				await updateMutation.mutateAsync({
					id: initialData.id,
					site_id: workType === "site" ? selectedSiteId || null : null,
					work_type: workType,
					hours,
					work_date: workDate,
					...(notes ? { notes } : {}),
				});
				toast.success("Zeiteintrag aktualisiert");
			} else {
				await createMutation.mutateAsync({
					site_id: workType === "site" ? selectedSiteId || null : null,
					work_type: workType,
					hours,
					work_date: workDate,
					...(notes ? { notes } : {}),
				});
				toast.success("Zeit erfasst");
			}
			onOpenChange(false);
		} catch {
			toast.error(
				mode === "edit"
					? "Aktualisierung fehlgeschlagen"
					: "Zeiterfassung fehlgeschlagen",
			);
		}
	};

	const handleDelete = async () => {
		if (!initialData) return;
		try {
			await deleteMutation.mutateAsync(initialData.id);
			toast.success("Zeiteintrag gelöscht");
			onOpenChange(false);
			setShowDeleteConfirm(false);
		} catch {
			toast.error("Löschen fehlgeschlagen");
		}
	};

	const isPending = createMutation.isPending || updateMutation.isPending;

	return (
		<>
			<Dialog open={open} onOpenChange={onOpenChange}>
				<DialogContent className="sm:max-w-md">
					<DialogHeader>
						<DialogTitle className="flex items-center gap-2.5 font-display">
							<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
								<Clock className="h-4 w-4 text-muted-foreground" />
							</div>
							{mode === "edit" ? "Zeit bearbeiten" : "Zeit buchen"}
						</DialogTitle>
						<DialogDescription>
							{mode === "edit"
								? "Zeiteintrag anpassen"
								: siteName || "Neuen Zeiteintrag erstellen"}
						</DialogDescription>
					</DialogHeader>

					<div className="space-y-5 py-4">
						<div className="space-y-2">
							<Label>Art der Arbeit</Label>
							<div className="flex flex-wrap gap-2">
								{workTypes.map((type) => (
									<Button
										key={type.value}
										variant={workType === type.value ? "default" : "outline"}
										size="sm"
										className={cn(
											"h-10 px-4",
											workType === type.value && "shadow-sm",
										)}
										onClick={() => setWorkType(type.value)}
									>
										{type.label}
									</Button>
								))}
							</div>
						</div>

						<div className="space-y-2">
							<Label>Datum</Label>
							<Input
								type="date"
								value={workDate}
								onChange={(e) => setWorkDate(e.target.value)}
								className="h-10"
							/>
						</div>

						{workType === "site" && (
							<div className="space-y-2">
								<Label>Baustelle</Label>
								<select
									className="flex h-10 w-full rounded-lg border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
									value={selectedSiteId}
									onChange={(event) => setSelectedSiteId(event.target.value)}
								>
									<option value="">Keine Zuordnung</option>
									{sites?.map((site) => (
										<option key={site.id} value={site.id}>
											{site.name}
										</option>
									))}
								</select>
							</div>
						)}

						<div className="space-y-2">
							<Label>Stunden</Label>
							<Input
								type="number"
								value={hours}
								onChange={(e) => {
									const value = parseFloat(e.target.value) || 0;
									setHours(value);
									if (touched.hours) {
										setHoursError(validateHours(value));
									}
								}}
								onBlur={() => {
									setTouched((prev) => ({ ...prev, hours: true }));
									setHoursError(validateHours(hours));
								}}
								min={0.5}
								max={24}
								step={0.5}
								className={cn("h-10", errors.hours && "border-destructive")}
							/>
							{errors.hours && (
								<p className="text-sm text-destructive">{errors.hours}</p>
							)}
							<div className="flex flex-wrap gap-2">
								{quickHours.map((h) => (
									<Button
										key={h}
										variant={hours === h ? "default" : "outline"}
										size="sm"
										onClick={() => {
											setHours(h);
											setHoursError(null);
										}}
										className={cn(
											"min-w-[48px] h-9",
											hours === h && "shadow-sm",
										)}
									>
										{h}h
									</Button>
								))}
							</div>
						</div>

						<div className="space-y-2">
							<Label>Notiz (optional)</Label>
							<Input
								placeholder="z.B. Montage Schränke"
								value={notes}
								onChange={(e) => setNotes(e.target.value)}
								className="h-10"
							/>
						</div>
					</div>

					<DialogFooter className="flex-col gap-2 sm:flex-row">
						{mode === "edit" && (
							<Button
								variant="destructive"
								onClick={() => setShowDeleteConfirm(true)}
								disabled={deleteMutation.isPending}
								className="h-10 w-full sm:w-auto"
							>
								Löschen
							</Button>
						)}
						<div className="flex gap-2 w-full sm:w-auto sm:ml-auto">
							<Button
								variant="outline"
								className="h-10 flex-1"
								onClick={() => onOpenChange(false)}
							>
								Abbrechen
							</Button>
							<Button
								className="h-10 shadow-sm flex-1"
								onClick={handleSubmit}
								disabled={!isFormValid || isPending}
							>
								{isPending
									? "Wird gespeichert..."
									: mode === "edit"
										? "Speichern"
										: "Buchen"}
							</Button>
						</div>
					</DialogFooter>
				</DialogContent>
			</Dialog>

			{/* Delete Confirmation Dialog */}
			<AlertDialog open={showDeleteConfirm} onOpenChange={setShowDeleteConfirm}>
				<AlertDialogContent>
					<AlertDialogHeader>
						<AlertDialogTitle>Zeiteintrag löschen</AlertDialogTitle>
						<AlertDialogDescription>
							Möchten Sie diesen Zeiteintrag wirklich löschen? Diese Aktion kann
							nicht rückgängig gemacht werden.
						</AlertDialogDescription>
					</AlertDialogHeader>
					<AlertDialogFooter>
						<AlertDialogCancel>Abbrechen</AlertDialogCancel>
						<AlertDialogAction
							onClick={handleDelete}
							className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
						>
							Löschen
						</AlertDialogAction>
					</AlertDialogFooter>
				</AlertDialogContent>
			</AlertDialog>
		</>
	);
}
