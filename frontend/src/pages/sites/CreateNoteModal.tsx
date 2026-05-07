import { ProjectTimelineComposer } from "./ProjectTimelineComposer";

interface CreateNoteModalProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	siteId: string;
	onSuccess: () => void;
	initialActivityType?: "note" | "photo";
}

export function CreateNoteModal({
	open,
	onOpenChange,
	siteId,
	onSuccess,
}: CreateNoteModalProps) {
	return (
		<ProjectTimelineComposer
			open={open}
			onOpenChange={onOpenChange}
			siteId={siteId}
			onSuccess={onSuccess}
			title="Dokumente hinzufügen"
			description="Fügen Sie eine Notiz hinzu und laden Sie Bilder oder PDFs in einem Eintrag hoch."
			submitLabel="Eintrag speichern"
			cameraMode={false}
		/>
	);
}
