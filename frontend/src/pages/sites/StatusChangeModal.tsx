import { ArrowRight, ArrowRightLeft } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
} from "@/components/ui/dialog";
import { useUpdateSite } from "@/lib/api/hooks";
import type { SiteStatus } from "@/types/sites";

interface StatusChangeModalProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	siteId: string;
	siteName: string;
	currentStatus: SiteStatus;
	onSuccess: () => void;
}

const statusLabels: Record<SiteStatus, string> = {
	planned: "Geplant",
	active: "Aktiv",
	completed: "Abgeschlossen",
	archived: "Archiviert",
};

export function StatusChangeModal({
	open,
	onOpenChange,
	siteId,
	siteName,
	currentStatus,
	onSuccess,
}: StatusChangeModalProps) {
	const [isChanging, setIsChanging] = useState(false);
	const updateSite = useUpdateSite();

	const getValidTransitions = (status: SiteStatus): SiteStatus[] => {
		switch (status) {
			case "planned":
				return ["active"];
			case "active":
				return ["completed"];
			case "completed":
				return ["archived"];
			case "archived":
				return [];
			default:
				return [];
		}
	};

	const handleStatusChange = async (newStatus: SiteStatus) => {
		setIsChanging(true);
		try {
			await updateSite.mutateAsync({
				id: siteId,
				status: newStatus,
			});
			toast.success(`Status geändert zu "${statusLabels[newStatus]}"`);
			onSuccess();
			onOpenChange(false);
		} catch (error) {
			const errorMessage =
				error instanceof Error ? error.message : "Unbekannter Fehler";

			if (
				errorMessage.includes("Invalid status transition") ||
				errorMessage.includes("bereits geändert")
			) {
				toast.error("Status wurde bereits geändert. Aktualisieren...", {
					duration: 2000,
				});
				setTimeout(() => {
					onSuccess();
				}, 1000);
			} else {
				toast.error("Fehler beim Ändern des Status");
			}
		} finally {
			setIsChanging(false);
		}
	};

	const validTransitions = getValidTransitions(currentStatus);

	return (
		<Dialog open={open} onOpenChange={onOpenChange}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 font-display">
						<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
							<ArrowRightLeft className="h-4 w-4 text-muted-foreground" />
						</div>
						Baustellen-Status ändern
					</DialogTitle>
					<DialogDescription>{siteName}</DialogDescription>
				</DialogHeader>

				<div className="space-y-5 py-2">
					<div className="flex items-center justify-center gap-4 py-4">
						<div className="flex items-center gap-2 rounded-lg bg-accent/40 px-4 py-2.5">
							<span className="text-sm font-medium">
								{statusLabels[currentStatus]}
							</span>
						</div>
						{validTransitions.length > 0 && (
							<>
								<ArrowRight className="h-4 w-4 text-muted-foreground" />
								<div className="flex gap-2">
									{validTransitions.map((targetStatus) => (
										<Button
											key={targetStatus}
											onClick={() => handleStatusChange(targetStatus)}
											disabled={isChanging}
											className="gap-2 h-10 shadow-sm"
										>
											{targetStatus === "active" && "Aktivieren"}
											{targetStatus === "completed" && "Abschließen"}
											{targetStatus === "archived" && "Archivieren"}
										</Button>
									))}
								</div>
							</>
						)}
					</div>

					{validTransitions.length === 0 && (
						<p className="text-sm text-muted-foreground text-center py-4">
							Keine weiteren Statusänderungen möglich
						</p>
					)}
				</div>
			</DialogContent>
		</Dialog>
	);
}
