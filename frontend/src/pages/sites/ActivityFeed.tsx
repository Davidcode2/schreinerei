import { ArrowRight, Camera, FileText, Package, Plus, Trash2 } from "lucide-react";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { toast } from "sonner";
import { DeleteConfirmDialog } from "@/components/shared/DeleteConfirmDialog";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { apiClient } from "@/lib/api/client";
import { useSiteMaterialHistory } from "@/lib/api/hooks";
import { useDeleteActivity } from "@/lib/api/hooks/useSites";
import type { Activity, ActivityAttachment } from "@/types/sites";
import {
	buildMediaViewerPath,
	extractAttachmentIdFromPhotoUrl,
} from "./mediaViewerRoute";

const statusLabels: Record<string, string> = {
	planned: "Geplant",
	active: "Aktiv",
	completed: "Abgeschlossen",
	archived: "Archiviert",
};

interface ActivityFeedProps {
	activities: Activity[];
	siteId: string;
	maxItems?: number;
}

function formatRelativeTime(dateString: string): string {
	const date = new Date(dateString);
	const now = new Date();
	const diffMs = now.getTime() - date.getTime();
	const diffMins = Math.round(diffMs / 60000);
	const diffHours = Math.round(diffMs / 3600000);
	const diffDays = Math.round(diffMs / 86400000);

	if (diffMins < 60) {
		return `vor ${diffMins} Min.`;
	}

	if (diffHours < 24) {
		return `vor ${diffHours} Std.`;
	}

	if (diffDays < 7) {
		return `vor ${diffDays} Tag${diffDays !== 1 ? "en" : ""}`;
	}

	return date.toLocaleDateString("de-DE", {
		day: "2-digit",
		month: "2-digit",
	});
}

function isProtectedAttachmentPath(url: string): boolean {
	return url.startsWith("/api/v1/attachments/");
}

function ImageAttachmentTile({
	attachment,
}: {
	attachment: ActivityAttachment;
}) {
	const [resolvedSrc, setResolvedSrc] = useState<string>(attachment.url);
	const [hasError, setHasError] = useState(false);

	useEffect(() => {
		if (!isProtectedAttachmentPath(attachment.url)) {
			setResolvedSrc(attachment.url);
			setHasError(false);
			return;
		}

		let objectUrl: string | null = null;
		let mounted = true;

		apiClient
			.getBlob(attachment.url)
			.then((blob) => {
				if (!mounted) {
					return;
				}

				objectUrl = URL.createObjectURL(blob);
				setResolvedSrc(objectUrl);
				setHasError(false);
			})
			.catch(() => {
				if (mounted) {
					setHasError(true);
					setResolvedSrc("");
				}
			});

		return () => {
			mounted = false;
			if (objectUrl) {
				URL.revokeObjectURL(objectUrl);
			}
		};
	}, [attachment.url]);

	if (hasError) {
		return (
			<div className="rounded-lg border bg-card p-3">
				<div className="flex aspect-square items-center justify-center rounded-lg bg-accent">
					<FileText className="h-5 w-5 text-muted-foreground" />
				</div>
				<p className="mt-2 truncate text-sm font-medium">
					{attachment.filename}
				</p>
				<p className="text-xs text-muted-foreground">
					Vorschau nicht verfügbar
				</p>
			</div>
		);
	}

	if (!resolvedSrc) {
		return null;
	}

	return (
		<div className="rounded-lg border bg-card p-2">
			<img
				src={resolvedSrc}
				alt={attachment.filename}
				className="aspect-square w-full rounded-lg object-cover"
			/>
			<p className="mt-2 truncate text-sm font-medium">{attachment.filename}</p>
		</div>
	);
}

function PdfAttachmentTile({ attachment }: { attachment: ActivityAttachment }) {
	return (
		<div className="rounded-lg border bg-card p-3">
			<div className="flex aspect-square items-center justify-center rounded-lg bg-accent">
				<div className="flex flex-col items-center gap-2 text-muted-foreground">
					<FileText className="h-6 w-6" />
					<span className="rounded-full bg-background px-2 py-1 text-[10px] font-semibold uppercase">
						PDF
					</span>
				</div>
			</div>
			<p className="mt-2 truncate text-sm font-medium">{attachment.filename}</p>
		</div>
	);
}

function AttachmentTile({ attachment }: { attachment: ActivityAttachment }) {
	if (attachment.mime_type === "application/pdf") {
		return <PdfAttachmentTile attachment={attachment} />;
	}

	return <ImageAttachmentTile attachment={attachment} />;
}

function getActivityHeading(activity: Activity) {
	if (activity.activity_type === "photo") {
		return "Foto hinzugefügt";
	}

	if (activity.activity_type === "status_change") {
		return "Status geändert";
	}

	return activity.attachments.length > 0 ? "Dokument hinzugefügt" : "Notiz";
}

function getStatusChangeSummary(content: string): string {
	try {
		const data = JSON.parse(content);
		const oldStatus = statusLabels[data.old_status] || data.old_status;
		const newStatus = statusLabels[data.new_status] || data.new_status;
		return `${oldStatus} → ${newStatus}`;
	} catch {
		return content;
	}
}

function buildViewerPath(
	siteId: string,
	activityId: string,
	attachmentId: string,
	filename: string,
): string {
	return buildMediaViewerPath(siteId, activityId, attachmentId, filename);
}

function ViewerTileLink({
	activity,
	attachment,
}: {
	activity: Activity;
	attachment: ActivityAttachment;
}) {
	const href = buildViewerPath(
		activity.site_id,
		activity.id,
		attachment.attachment_id,
		attachment.filename,
	);

	return (
		<Link
			aria-label={`Medium öffnen: ${attachment.filename}`}
			className="block rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2"
			to={href}
		>
			<div className="rounded-lg transition-colors hover:bg-accent/40 hover:ring-1 hover:ring-primary/30">
				<AttachmentTile attachment={attachment} />
			</div>
		</Link>
	);
}

function buildLegacyPhotoAttachment(
	activity: Activity,
): ActivityAttachment | null {
	if (!activity.photo_url) {
		return null;
	}

	const attachmentId = extractAttachmentIdFromPhotoUrl(activity.photo_url);
	if (!attachmentId) {
		return null;
	}

	return {
		attachment_id: attachmentId,
		filename: "Aktivitätsfoto",
		mime_type: "image/jpeg",
		url: activity.photo_url,
		thumbnail_url: activity.photo_url,
	};
}

function getDeleteItemName(activity: Activity): string {
	if (activity.activity_type === "photo") {
		return "dieses Foto";
	}

	return activity.attachments.length > 0 ? "diesen Eintrag" : "diese Notiz";
}

function ActivityCard({
	activity,
	onDelete,
}: {
	activity: Activity;
	onDelete: (activity: Activity) => void;
}) {
	const hasDocumentAttachments =
		activity.activity_type !== "photo" &&
		activity.activity_type !== "status_change" &&
		activity.attachments.length > 0;
	const photoAttachment =
		activity.activity_type === "photo"
			? buildLegacyPhotoAttachment(activity)
			: null;

	return (
		<Card className="overflow-hidden transition-all duration-200 hover:shadow-sm">
			<div className="flex gap-3 p-4">
				<div className="flex-shrink-0">
					{activity.activity_type === "photo" ? (
						<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
							<Camera className="h-4 w-4 text-muted-foreground" />
						</div>
					) : activity.activity_type === "status_change" ? (
						<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-success/15">
							<ArrowRight className="h-4 w-4 text-success" />
						</div>
					) : (
						<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
							<FileText className="h-4 w-4 text-muted-foreground" />
						</div>
					)}
				</div>

				<div className="min-w-0 flex-1">
					<div className="flex items-start justify-between gap-2">
						<p className="text-sm font-medium">
							{getActivityHeading(activity)}
						</p>
						<div className="flex items-center gap-2">
							<span className="whitespace-nowrap text-xs text-muted-foreground">
								{formatRelativeTime(activity.created_at)}
							</span>
							{activity.can_delete ? (
								<Button
									aria-label={`Eintrag löschen: ${activity.id}`}
									variant="ghost"
									size="icon"
									className="h-8 w-8 text-muted-foreground hover:text-destructive"
									onClick={() => onDelete(activity)}
								>
									<Trash2 className="h-4 w-4" />
								</Button>
							) : null}
						</div>
					</div>

					{activity.activity_type === "status_change" && activity.content ? (
						<p className="mt-1 break-all text-sm text-muted-foreground">
							{getStatusChangeSummary(activity.content)}
						</p>
					) : null}

					{activity.content && activity.activity_type !== "status_change" ? (
						<p className="mt-1 break-all text-sm text-muted-foreground">
							{activity.content}
						</p>
					) : null}

					{hasDocumentAttachments ? (
						<div className="mt-3 grid grid-cols-2 gap-3 md:grid-cols-3">
							{activity.attachments.map((attachment) => (
								<ViewerTileLink
									key={attachment.attachment_id}
									activity={activity}
									attachment={attachment}
								/>
							))}
						</div>
					) : null}

					{photoAttachment ? (
						<div className="mt-3 max-w-xs">
							<ViewerTileLink
								activity={activity}
								attachment={photoAttachment}
							/>
						</div>
					) : null}
				</div>
			</div>
		</Card>
	);
}

export function ActivityFeed({
	activities,
	siteId,
	maxItems,
}: ActivityFeedProps) {
	const [activeTab, setActiveTab] = useState("notes");
	const [deleteTarget, setDeleteTarget] = useState<Activity | null>(null);
	const { data: materialHistory, isLoading: isMaterialHistoryLoading } =
		useSiteMaterialHistory(siteId);
	const deleteActivityMutation = useDeleteActivity();

	const noteActivities = activities.filter(
		(activity) =>
			activity.activity_type === "photo" ||
			activity.activity_type === "note" ||
			activity.activity_type === "status_change",
	);

	const displayedActivities = maxItems
		? noteActivities.slice(0, maxItems)
		: noteActivities;

	async function handleDeleteConfirm() {
		if (!deleteTarget) {
			return;
		}

		try {
			await deleteActivityMutation.mutateAsync({
				siteId,
				activityId: deleteTarget.id,
			});
			toast.success("Eintrag gelöscht");
			setDeleteTarget(null);
		} catch (error) {
			const message =
				error instanceof Error
					? error.message
					: "Eintrag konnte nicht gelöscht werden";
			toast.error(message);
		}
	}

	return (
		<>
			<Tabs value={activeTab} onValueChange={setActiveTab}>
				<TabsList>
					<TabsTrigger value="notes">Notizen/Dokumente</TabsTrigger>
					<TabsTrigger value="materials">Material</TabsTrigger>
				</TabsList>

				<TabsContent value="notes" className="mt-4 space-y-4">
					{displayedActivities.length === 0 ? (
						<div className="py-8 text-center">
							<div className="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-2xl bg-accent/60">
								<FileText className="h-6 w-6 text-muted-foreground" />
							</div>
							<p className="text-sm text-muted-foreground">
								Noch keine Dokumente oder Notizen
							</p>
							<p className="mt-1 text-xs text-muted-foreground">
								Fügen Sie eine Notiz hinzu oder laden Sie Bilder/PDFs hoch, um
								den Verlauf dieser Baustelle zu starten.
							</p>
						</div>
					) : (
						<>
							{displayedActivities.map((activity) => (
								<ActivityCard
									key={activity.id}
									activity={activity}
									onDelete={setDeleteTarget}
								/>
							))}
							{noteActivities.length > (maxItems || 0) && maxItems ? (
								<p className="text-center text-xs text-muted-foreground">
									+{noteActivities.length - maxItems} weitere Aktivitäten
								</p>
							) : null}
						</>
					)}
				</TabsContent>

				<TabsContent value="materials" className="mt-0">
					<div className="mt-4">
						<Button asChild className="h-10 gap-2 shadow-sm w-full sm:w-auto">
							<Link to="/inventory">
								<Package className="h-4 w-4" />
								Material buchen
							</Link>
						</Button>
					</div>
					{isMaterialHistoryLoading ? (
						<p className="py-8 text-center text-sm text-muted-foreground">
							Material-Historie wird geladen...
						</p>
					) : materialHistory && materialHistory.length > 0 ? (
						<div className="mt-4 space-y-3">
							{materialHistory.map((entry) => (
								<Card
									key={entry.id}
									className="overflow-hidden transition-all duration-200 hover:shadow-sm"
								>
									<div className="p-4">
										<div className="flex items-start justify-between gap-3">
											<div>
												<Link
													to={`/inventory/${entry.material_id}`}
													className="text-sm font-medium text-foreground transition-colors hover:text-primary hover:underline"
												>
													{entry.material_name}
												</Link>
												<p className="text-xs text-muted-foreground">
													Kategorie: {entry.category_name}
												</p>
												<p className="text-xs text-muted-foreground">
													Entnommen von: {entry.extracted_by}
												</p>
												{entry.site_id && entry.site_name ? (
													<Link
														to={`/sites/${entry.site_id}`}
														className="text-xs text-primary hover:underline"
													>
														{entry.site_name}
													</Link>
												) : null}
											</div>
											<div className="text-right shrink-0">
												<p className="text-sm font-medium">
													{entry.quantity_change < 0
														? `Entnahme ${Math.abs(entry.quantity_change)}`
														: `+${entry.quantity_change}`}
												</p>
												<p className="text-xs text-muted-foreground">
													{formatRelativeTime(entry.created_at)}
												</p>
											</div>
										</div>
									</div>
									<div className="flex justify-end border-t border-border/60 px-4 py-2.5 bg-accent/30">
										<Button
											asChild
											variant="ghost"
											size="sm"
											className="gap-1.5 text-xs text-muted-foreground hover:text-primary h-8"
											aria-label={`Mehr von ${entry.material_name} entnehmen`}
										>
											<Link
												to={`/inventory/${entry.material_id}?action=withdraw&siteId=${siteId}`}
											>
												<Plus className="h-3.5 w-3.5" />
												Entnehmen
											</Link>
										</Button>
									</div>
								</Card>
							))}
						</div>
					) : (
						<div className="py-8 text-center">
							<p className="text-sm text-muted-foreground">
								Noch keine Materialentnahmen für diese Baustelle
							</p>
						</div>
					)}
				</TabsContent>
			</Tabs>

			<DeleteConfirmDialog
				open={deleteTarget !== null}
				onOpenChange={(open) => {
					if (!open && !deleteActivityMutation.isPending) {
						setDeleteTarget(null);
					}
				}}
				onConfirm={handleDeleteConfirm}
				itemName={
					deleteTarget ? getDeleteItemName(deleteTarget) : "diesen Eintrag"
				}
				isPending={deleteActivityMutation.isPending}
			/>
		</>
	);
}
