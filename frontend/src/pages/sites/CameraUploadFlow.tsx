import { ProjectTimelineComposer } from "./ProjectTimelineComposer";

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
	return (
		<ProjectTimelineComposer
			open={open}
			onOpenChange={onOpenChange}
			siteId={siteId}
			onSuccess={onSuccess}
			title="Foto hinzufügen"
			description="Nehmen Sie ein Foto auf oder ergänzen Sie den Eintrag direkt mit weiteren Dateien."
			submitLabel="Eintrag speichern"
			cameraMode
		/>
	);
}
