import { Download, FileText, Link2, X } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogTitle,
} from "@/components/ui/dialog";
import { Separator } from "@/components/ui/separator";
import { apiClient } from "@/lib/api/client";
import type { MediaViewerTarget } from "./mediaViewerRoute";

interface MediaViewerProps {
	open: boolean;
	target: MediaViewerTarget | null;
	sharePath: string;
	onClose: () => void;
}

const PREVIEW_ERROR_COPY =
	"Medium konnte nicht geladen werden. Prüfen Sie Ihre Verbindung und öffnen Sie die Datei erneut.";

function formatViewerTimestamp(date: string): string {
	return `${new Date(date).toLocaleString("de-DE", {
		day: "2-digit",
		month: "2-digit",
		year: "numeric",
		hour: "2-digit",
		minute: "2-digit",
	})} Uhr`;
}

function fileTypeLabel(mimeType: string): string {
	if (mimeType === "application/pdf") {
		return "PDF";
	}

	if (mimeType.startsWith("image/")) {
		return "Bild";
	}

	return mimeType;
}

function MediaPreviewPane({
	target,
	onClose,
}: {
	target: MediaViewerTarget | null;
	onClose: () => void;
}) {
	const [previewUrl, setPreviewUrl] = useState<string | null>(null);
	const [hasPreviewError, setHasPreviewError] = useState(false);

	useEffect(() => {
		if (!target) {
			return;
		}

		let active = true;
		let objectUrl: string | null = null;

		apiClient
			.getBlob(target.attachment.url)
			.then((blob) => {
				if (!active) {
					return;
				}

				objectUrl = URL.createObjectURL(blob);
				setPreviewUrl(objectUrl);
			})
			.catch(() => {
				if (active) {
					setHasPreviewError(true);
				}
			});

		return () => {
			active = false;
			if (objectUrl) {
				URL.revokeObjectURL(objectUrl);
			}
		};
	}, [target]);

	if (!target || hasPreviewError) {
		return (
			<div className="flex h-full min-h-[320px] flex-col items-center justify-center gap-4 rounded-xl bg-accent/40 p-6 text-center">
				<div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-accent/60">
					<FileText className="h-6 w-6 text-muted-foreground" />
				</div>
				<p className="max-w-sm text-sm text-muted-foreground">
					{PREVIEW_ERROR_COPY}
				</p>
				<Button variant="outline" className="h-10" onClick={onClose}>
					Schließen
				</Button>
			</div>
		);
	}

	if (!previewUrl) {
		return (
			<div className="flex h-full min-h-[320px] items-center justify-center rounded-xl bg-accent/40">
				<div className="h-10 w-10 animate-spin rounded-full border-2 border-muted border-t-primary" />
			</div>
		);
	}

	if (target.attachment.mime_type === "application/pdf") {
		return (
			<div className="h-full min-h-[320px] rounded-xl bg-card p-4">
				<iframe
					className="h-[60dvh] w-full rounded-lg border"
					src={previewUrl}
					title={`Dokumentvorschau: ${target.attachment.filename}`}
				/>
			</div>
		);
	}

	return (
		<div className="flex h-full min-h-[320px] items-center justify-center rounded-xl bg-primary-foreground p-4">
			<img
				alt={target.attachment.filename}
				className="max-h-[70dvh] w-full object-contain"
				src={previewUrl}
			/>
		</div>
	);
}

export function MediaViewer({
	open,
	target,
	sharePath,
	onClose,
}: MediaViewerProps) {
	const shareUrl = useMemo(
		() => `${window.location.origin}${sharePath}`,
		[sharePath],
	);

	const copyShareLink = async () => {
		try {
			await navigator.clipboard.writeText(shareUrl);
			toast.success("Link kopiert");
		} catch {
			toast.error("Link konnte nicht kopiert werden");
		}
	};

	const downloadAttachment = async () => {
		if (!target) {
			return;
		}

		const blob = await apiClient.getBlob(target.attachment.url);
		const downloadUrl = URL.createObjectURL(blob);
		const link = document.createElement("a");
		link.href = downloadUrl;
		link.download = target.attachment.filename || "Aktivitätsfoto";
		link.click();
		URL.revokeObjectURL(downloadUrl);
	};

	return (
		<Dialog open={open} onOpenChange={(nextOpen) => !nextOpen && onClose()}>
			<DialogContent
				showCloseButton={false}
				className="h-[100dvh] w-[100dvw] max-w-none translate-x-[-50%] translate-y-[-50%] gap-0 rounded-none border-0 p-0 sm:h-[calc(100dvh-32px)] sm:w-[calc(100dvw-32px)] sm:rounded-xl"
			>
				<DialogTitle className="sr-only">Medienansicht</DialogTitle>
				<DialogDescription className="sr-only">
					Vollbildansicht für Baustellenmedien mit Metadaten und Aktionen.
				</DialogDescription>

				<div className="grid h-full grid-cols-1 bg-background lg:grid-cols-[minmax(0,1fr)_320px]">
					<div className="min-h-0 bg-primary-foreground/95 p-4 lg:p-6">
						<MediaPreviewPane
							key={
								target
									? `${target.activity.id}:${target.attachment.attachment_id}`
									: "empty"
							}
							target={open ? target : null}
							onClose={onClose}
						/>
					</div>

					<aside className="flex flex-col gap-6 border-l bg-card p-5 lg:p-6">
						<div className="flex items-start justify-between gap-3">
							<div className="flex flex-1 flex-col gap-2">
								<Button
									className="h-10 w-full gap-2 justify-start shadow-sm"
									onClick={copyShareLink}
								>
									<Link2 className="h-4 w-4" />
									Link kopieren
								</Button>
								<Button
									className="h-10 w-full gap-2 justify-start"
									onClick={downloadAttachment}
									variant="outline"
								>
									<Download className="h-4 w-4" />
									Herunterladen
								</Button>
							</div>
							<Button
								aria-label="Viewer schließen"
								className="h-10 w-10 shrink-0"
								onClick={onClose}
								size="icon"
								variant="ghost"
							>
								<X className="h-5 w-5" />
							</Button>
						</div>

						<div className="space-y-2">
							<h2 className="text-lg font-display">
								{target?.title ?? "Medium"}
							</h2>
							{target ? (
								<div className="space-y-1 text-sm text-muted-foreground">
									<p>{target.activity.creator_name}</p>
									<p>{formatViewerTimestamp(target.activity.created_at)}</p>
								</div>
							) : null}
						</div>

						<Separator />

						<section className="space-y-2">
							<h3 className="text-xs font-medium uppercase tracking-widest text-muted-foreground">
								Notiz
							</h3>
							<p className="text-sm text-muted-foreground leading-relaxed">
								{target?.activity.content?.trim() || "Keine Notiz vorhanden."}
							</p>
						</section>

						<section className="space-y-2">
							<h3 className="text-xs font-medium uppercase tracking-widest text-muted-foreground">
								Dateidetails
							</h3>
							<div className="space-y-1 text-sm">
								<p className="font-medium">
									{target?.attachment.filename ?? "Aktivitätsfoto"}
								</p>
								<p className="text-muted-foreground">
									{target
										? fileTypeLabel(target.attachment.mime_type)
										: "Unbekannt"}
								</p>
							</div>
						</section>
					</aside>
				</div>
			</DialogContent>
		</Dialog>
	);
}
