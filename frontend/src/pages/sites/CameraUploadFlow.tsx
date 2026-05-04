import { Camera } from "lucide-react";
import { useEffect, useRef, useState } from "react";
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
import { useCreateActivity, useUploadSitePhoto } from "@/lib/api/hooks";
import { queuePhotoUploadAction } from "@/lib/offline/queue";
import { isOnline } from "@/lib/offline/sync";

const MAX_UPLOAD_SIZE_BYTES = 10 * 1024 * 1024;

interface CameraUploadFlowProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	siteId: string;
	onSuccess: () => void;
}

export function CameraUploadFlow({
	open,
	onOpenChange,
	siteId,
	onSuccess,
}: CameraUploadFlowProps) {
	const [selectedFile, setSelectedFile] = useState<File | null>(null);
	const [note, setNote] = useState("");
	const [previewUrl, setPreviewUrl] = useState<string | null>(null);
	const fileInputRef = useRef<HTMLInputElement>(null);
	const pickerTriggeredRef = useRef(false);
	const createActivity = useCreateActivity();
	const uploadPhoto = useUploadSitePhoto();

	useEffect(() => {
		if (open && !pickerTriggeredRef.current) {
			requestAnimationFrame(() => fileInputRef.current?.click());
			pickerTriggeredRef.current = true;
		}
		if (!open) {
			pickerTriggeredRef.current = false;
		}
	}, [open]);

	useEffect(() => {
		return () => {
			if (previewUrl) {
				URL.revokeObjectURL(previewUrl);
			}
		};
	}, [previewUrl]);

	const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
		const file = event.target.files?.[0];
		if (!file) {
			onOpenChange(false);
			return;
		}

		if (!file.type.startsWith("image/")) {
			toast.error("Bitte wählen Sie eine Bilddatei aus");
			onOpenChange(false);
			return;
		}

		if (file.size > MAX_UPLOAD_SIZE_BYTES) {
			toast.error("Das Foto ist zu groß. Maximal 10 MB sind erlaubt.");
			onOpenChange(false);
			return;
		}

		if (previewUrl) {
			URL.revokeObjectURL(previewUrl);
		}
		setSelectedFile(file);
		setPreviewUrl(URL.createObjectURL(file));
	};

	const handleSubmit = async () => {
		if (!selectedFile) {
			toast.error("Bitte wählen Sie ein Foto aus");
			return;
		}

		try {
			const trimmedNote = note.trim() || undefined;

			if (!isOnline()) {
				await queuePhotoUploadAction(
					trimmedNote
						? {
								siteId,
								file: selectedFile,
								content: trimmedNote,
							}
						: {
								siteId,
								file: selectedFile,
							},
				);

				toast.success("Foto offline gespeichert", {
					description:
						"Wird automatisch hochgeladen, sobald Sie wieder online sind.",
				});
				resetForm();
				onSuccess();
				onOpenChange(false);
				return;
			}

			const uploadResponse = await uploadPhoto.mutateAsync({
				siteId,
				file: selectedFile,
			});

			await createActivity.mutateAsync({
				siteId,
				activity_type: "photo",
				photo_url: uploadResponse.photo_url,
				...(trimmedNote ? { content: trimmedNote } : {}),
			});

			toast.success("Foto hinzugefügt");
			resetForm();
			onSuccess();
			onOpenChange(false);
		} catch {
			toast.error("Foto konnte nicht hochgeladen werden");
		}
	};

	const resetForm = () => {
		if (previewUrl) {
			URL.revokeObjectURL(previewUrl);
		}
		setSelectedFile(null);
		setNote("");
		setPreviewUrl(null);
		if (fileInputRef.current) {
			fileInputRef.current.value = "";
		}
	};

	const handleOpenChange = (newOpen: boolean) => {
		if (!newOpen) {
			resetForm();
		}
		onOpenChange(newOpen);
	};

	const isPending = createActivity.isPending || uploadPhoto.isPending;
	const showDialog = selectedFile !== null;

	return (
		<>
			<input
				ref={fileInputRef}
				type="file"
				accept="image/*"
				capture="environment"
				className="hidden"
				onChange={handleFileChange}
				data-testid="camera-file-input"
			/>

			{showDialog && (
				<Dialog open={open} onOpenChange={handleOpenChange}>
					<DialogContent className="sm:max-w-md">
						<DialogHeader>
							<DialogTitle className="flex items-center gap-2.5 font-display">
								<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
									<Camera className="h-4 w-4 text-muted-foreground" />
								</div>
								Foto hinzufügen
							</DialogTitle>
							<DialogDescription>
								Fügen Sie optional eine Notiz zu Ihrem Foto hinzu
							</DialogDescription>
						</DialogHeader>

						<div className="py-4 space-y-5">
							{previewUrl && (
								<div className="relative overflow-hidden rounded-xl bg-accent/40 p-2">
									<img
										src={previewUrl}
										alt="Vorschau"
										className="w-full max-h-64 object-contain rounded-lg"
									/>
								</div>
							)}

							<Textarea
								placeholder="Notiz hinzufügen (optional)..."
								value={note}
								onChange={(e) => setNote(e.target.value)}
								rows={2}
								className="resize-none"
							/>
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
								disabled={isPending || !selectedFile}
							>
								{isPending ? "Hochladen..." : "Hochladen"}
							</Button>
						</DialogFooter>
					</DialogContent>
				</Dialog>
			)}
		</>
	);
}
