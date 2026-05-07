import { Camera, FileText, Plus, X } from "lucide-react";
import { type ChangeEvent, useEffect, useMemo, useRef, useState } from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from "@/components/ui/dialog";
import { Textarea } from "@/components/ui/textarea";
import { useCreateActivity, useUploadSiteAttachment } from "@/lib/api/hooks";

const ACCEPTED_MIME_TYPES = [
	"image/jpeg",
	"image/png",
	"image/webp",
	"application/pdf",
] as const;

const MAX_UPLOAD_SIZE_BYTES = 10 * 1024 * 1024;

type ComposerFile = {
	id: string;
	file: File;
	previewUrl: string | null;
};

interface ProjectTimelineComposerProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	siteId: string;
	onSuccess: () => void;
	title?: string;
	description?: string;
	submitLabel?: string;
	cameraMode?: boolean;
}

function isAcceptedFile(file: File) {
	return ACCEPTED_MIME_TYPES.includes(
		file.type as (typeof ACCEPTED_MIME_TYPES)[number],
	);
}

function isWithinUploadLimit(file: File) {
	return file.size <= MAX_UPLOAD_SIZE_BYTES;
}

function buildComposerFile(file: File): ComposerFile {
	return {
		id: `${file.name}-${file.lastModified}-${file.size}`,
		file,
		previewUrl: file.type.startsWith("image/")
			? URL.createObjectURL(file)
			: null,
	};
}

function revokePreview(file: ComposerFile) {
	if (file.previewUrl) {
		URL.revokeObjectURL(file.previewUrl);
	}
}

function getSelectionError(files: File[]) {
	const acceptedFiles = files.filter(isAcceptedFile);
	const invalidFiles = files.filter((file) => !isAcceptedFile(file));
	const oversizedFiles = acceptedFiles.filter((file) => !isWithinUploadLimit(file));

	if (invalidFiles.length === 0 && oversizedFiles.length === 0) {
		return null;
	}

	return [
		invalidFiles.length > 0
			? `Nicht unterstützte Dateien: ${invalidFiles
					.map((file) => `${file.name} (${file.type || "unbekannt"})`)
					.join(", ")}`
			: null,
		oversizedFiles.length > 0
			? `Zu groß (max. 10 MB): ${oversizedFiles.map((file) => file.name).join(", ")}`
			: null,
	]
		.filter(Boolean)
		.join(" ");
}

function buildValidComposerFiles(files: File[]) {
	return files.filter(isAcceptedFile).filter(isWithinUploadLimit).map(buildComposerFile);
}

export function ProjectTimelineComposer({
	open,
	onOpenChange,
	siteId,
	onSuccess,
	title = "Projekt-Timeline-Eintrag",
	description = "Fügen Sie eine Notiz hinzu und laden Sie Bilder oder PDFs in einem Eintrag hoch.",
	submitLabel = "Eintrag speichern",
	cameraMode = false,
}: ProjectTimelineComposerProps) {
	const [content, setContent] = useState("");
	const [selectedFiles, setSelectedFiles] = useState<ComposerFile[]>([]);
	const [selectionError, setSelectionError] = useState<string | null>(null);
	const fileInputRef = useRef<HTMLInputElement | null>(null);
	const cameraInputRef = useRef<HTMLInputElement | null>(null);
	const autoCameraPendingRef = useRef(false);
	const createActivity = useCreateActivity();
	const uploadSiteAttachment = useUploadSiteAttachment();

	useEffect(() => {
		return () => {
			selectedFiles.forEach(revokePreview);
		};
	}, [selectedFiles]);

	useEffect(() => {
		if (!open || !cameraMode || autoCameraPendingRef.current) {
			if (!open) {
				autoCameraPendingRef.current = false;
			}
			return;
		}

		autoCameraPendingRef.current = true;
		requestAnimationFrame(() => cameraInputRef.current?.click());
	}, [cameraMode, open]);

	const isPending = createActivity.isPending || uploadSiteAttachment.isPending;
	const hasContent = content.trim().length > 0;
	const canSubmit = hasContent || selectedFiles.length > 0;

	const helperText = useMemo(() => {
		if (!selectionError) {
			return "Unterstützt Bilder und PDFs bis 10 MB.";
		}

		return selectionError;
	}, [selectionError]);

	function resetForm() {
		selectedFiles.forEach(revokePreview);
		setContent("");
		setSelectedFiles([]);
		setSelectionError(null);
		autoCameraPendingRef.current = false;

		if (fileInputRef.current) {
			fileInputRef.current.value = "";
		}

		if (cameraInputRef.current) {
			cameraInputRef.current.value = "";
		}
	}

	function handleOpenChange(nextOpen: boolean) {
		if (!nextOpen) {
			resetForm();
		}

		onOpenChange(nextOpen);
	}

	function appendFiles(files: File[]) {
		if (files.length === 0) {
			return;
		}

		const validFiles = buildValidComposerFiles(files);
		if (validFiles.length > 0) {
			setSelectedFiles((currentFiles) => [...currentFiles, ...validFiles]);
		}

		setSelectionError(getSelectionError(files));
	}

	function handleStandardSelection(event: ChangeEvent<HTMLInputElement>) {
		appendFiles(Array.from(event.target.files ?? []));
		event.target.value = "";
	}

	function handleCameraSelection(event: ChangeEvent<HTMLInputElement>) {
		const files = Array.from(event.target.files ?? []);
		const hadFilesBeforeSelection = selectedFiles.length > 0;
		const autoCameraPending = autoCameraPendingRef.current;
		autoCameraPendingRef.current = false;

		if (files.length === 0) {
			if (autoCameraPending && !hadFilesBeforeSelection && !hasContent) {
				handleOpenChange(false);
			}
			return;
		}

		appendFiles(files);
		event.target.value = "";
	}

	function handleRemoveFile(fileId: string) {
		setSelectedFiles((currentFiles) => {
			const fileToRemove = currentFiles.find((file) => file.id === fileId);
			if (fileToRemove) {
				revokePreview(fileToRemove);
			}

			return currentFiles.filter((file) => file.id !== fileId);
		});
	}

	async function handleSubmit() {
		if (!canSubmit) {
			toast.error(
				"Bitte fügen Sie eine Notiz oder mindestens eine Datei hinzu",
			);
			return;
		}

		try {
			const uploadedAttachments = await Promise.all(
				selectedFiles.map(({ file }) =>
					uploadSiteAttachment.mutateAsync({ siteId, file }),
				),
			);

			const trimmedContent = content.trim();
			await createActivity.mutateAsync({
				siteId,
				activity_type: "note",
				attachment_ids: uploadedAttachments.map(
					(attachment) => attachment.attachment_id,
				),
				...(trimmedContent ? { content: trimmedContent } : {}),
			});

			toast.success("Eintrag gespeichert");
			resetForm();
			onSuccess();
			onOpenChange(false);
		} catch {
			toast.error(
				"Upload fehlgeschlagen. Prüfen Sie Dateityp oder Verbindung und versuchen Sie es erneut.",
			);
		}
	}

	return (
		<Dialog open={open} onOpenChange={handleOpenChange}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 font-display">
						<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
							<FileText className="h-4 w-4 text-muted-foreground" />
						</div>
						{title}
					</DialogTitle>
					<DialogDescription>{description}</DialogDescription>
				</DialogHeader>

				<div className="space-y-5 py-4">
					<Textarea
						placeholder="Notiz hinzufügen (optional)..."
						value={content}
						onChange={(event) => setContent(event.target.value)}
						rows={2}
						className="max-h-40 min-h-20 resize-y"
					/>

					<div className="space-y-2">
						<div className="flex flex-wrap gap-2">
							<input
								ref={fileInputRef}
								type="file"
								multiple
								accept="image/jpeg,image/png,image/webp,application/pdf"
								className="sr-only"
								onChange={handleStandardSelection}
								data-testid="timeline-file-input"
							/>
							<input
								ref={cameraInputRef}
								type="file"
								accept="image/*"
								capture="environment"
								className="sr-only"
								onChange={handleCameraSelection}
								data-testid="timeline-camera-input"
							/>
							<Button
								type="button"
								variant="outline"
								className="h-10 gap-2"
								onClick={() => fileInputRef.current?.click()}
							>
								<Plus className="h-4 w-4" />
								Dateien auswählen
							</Button>
							<Button
								type="button"
								variant="outline"
								className="h-10 gap-2"
								onClick={() => cameraInputRef.current?.click()}
							>
								<Camera className="h-4 w-4" />
								Foto aufnehmen
							</Button>
						</div>
					</div>

					<div className="space-y-2">
						{selectedFiles.map((selectedFile) => (
							<div
								key={selectedFile.id}
								className="flex items-center gap-3 rounded-lg border bg-card p-3"
							>
								<div className="flex h-14 w-14 flex-shrink-0 items-center justify-center overflow-hidden rounded-lg bg-accent">
									{selectedFile.previewUrl ? (
										<img
											src={selectedFile.previewUrl}
											alt={selectedFile.file.name}
											className="h-full w-full object-cover"
										/>
									) : (
										<div className="flex flex-col items-center gap-1 text-muted-foreground">
											<FileText className="h-5 w-5" />
											<span className="text-[10px] font-semibold uppercase">
												PDF
											</span>
										</div>
									)}
								</div>

								<div className="min-w-0 flex-1">
									<p className="truncate text-sm font-medium">
										{selectedFile.file.name}
									</p>
									<p className="text-xs text-muted-foreground">
										{selectedFile.file.type.startsWith("image/")
											? "Bild"
											: "PDF"}
									</p>
								</div>

								<Button
									type="button"
									variant="ghost"
									size="icon"
									className="h-9 w-9 flex-shrink-0 text-muted-foreground hover:text-destructive"
									onClick={() => handleRemoveFile(selectedFile.id)}
									aria-label={`Datei entfernen: ${selectedFile.file.name}`}
								>
									<X className="h-4 w-4" />
								</Button>
							</div>
						))}
					</div>

					<p className="text-xs text-muted-foreground">{helperText}</p>
				</div>

				<DialogFooter>
					<Button
						variant="outline"
						className="h-10"
						onClick={() => handleOpenChange(false)}
					>
						Abbrechen
					</Button>
					<Button
						className="h-10 shadow-sm"
						onClick={handleSubmit}
						disabled={isPending || !canSubmit}
					>
						{isPending ? "Speichern..." : submitLabel}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
