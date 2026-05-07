import { beforeEach, describe, expect, it, vi } from "vitest";
import userEvent from "@testing-library/user-event";
import { render, screen } from "@/test/utils";
import { ProjectTimelineComposer } from "./ProjectTimelineComposer";

const createActivityMutate = vi.fn();
const uploadSiteAttachmentMutate = vi.fn();

vi.mock("@/lib/api/hooks", () => ({
	useCreateActivity: () => ({ isPending: false, mutateAsync: createActivityMutate }),
	useUploadSiteAttachment: () => ({
		isPending: false,
		mutateAsync: uploadSiteAttachmentMutate,
	}),
}));

function renderComposer(cameraMode = false) {
	render(
		<ProjectTimelineComposer
			open
			onOpenChange={vi.fn()}
			siteId="site-1"
			onSuccess={vi.fn()}
			cameraMode={cameraMode}
		/>,
	);
}

describe("ProjectTimelineComposer", () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it("stages note text plus multiple images and pdfs in one composer session", async () => {
		renderComposer();
		const user = userEvent.setup({ applyAccept: false });

		const picker = document.querySelector(
			'input[data-testid="timeline-file-input"]',
		) as HTMLInputElement;
		const imageFile = new File(["image"], "planung.jpg", { type: "image/jpeg" });
		const pdfFile = new File(["pdf"], "angebot.pdf", {
			type: "application/pdf",
		});

		await user.type(
			screen.getByPlaceholderText("Notiz hinzufügen (optional)..."),
			"Montage abgeschlossen",
		);
		await user.upload(picker, [imageFile, pdfFile]);

		expect(screen.getByDisplayValue("Montage abgeschlossen")).toBeInTheDocument();
		expect(screen.getByText("planung.jpg")).toBeInTheDocument();
		expect(screen.getByText("angebot.pdf")).toBeInTheDocument();
	});

	it("keeps camera-picked images in the same selected file list as manual documents", async () => {
		renderComposer(true);
		const user = userEvent.setup({ applyAccept: false });

		const cameraInput = document.querySelector(
			'input[data-testid="timeline-camera-input"]',
		) as HTMLInputElement;
		const picker = document.querySelector(
			'input[data-testid="timeline-file-input"]',
		) as HTMLInputElement;
		const imageFile = new File(["image"], "kamera.jpg", { type: "image/jpeg" });
		const pdfFile = new File(["pdf"], "angebot.pdf", {
			type: "application/pdf",
		});

		await user.upload(cameraInput, imageFile);
		await user.upload(picker, pdfFile);

		expect(screen.getByText("kamera.jpg")).toBeInTheDocument();
		expect(screen.getByText("angebot.pdf")).toBeInTheDocument();
	});

	it("rejects invalid mime types and files above the 10 MB limit", async () => {
		renderComposer();
		const user = userEvent.setup({ applyAccept: false });

		const picker = document.querySelector(
			'input[data-testid="timeline-file-input"]',
		) as HTMLInputElement;
		const invalidFile = new File(["text"], "readme.txt", { type: "text/plain" });
		const oversizedPdf = new File(
			[new Uint8Array(10 * 1024 * 1024 + 1)],
			"gross.pdf",
			{ type: "application/pdf" },
		);

		await user.upload(picker, [invalidFile, oversizedPdf]);

		expect(screen.getByText(/Nicht unterstützte Dateien:/i)).toBeInTheDocument();
		expect(screen.getByText(/Zu groß \(max\. 10 MB\): gross\.pdf/i)).toBeInTheDocument();
		expect(screen.queryByText("readme.txt")).not.toBeInTheDocument();
		expect(screen.queryByText("gross.pdf")).not.toBeInTheDocument();
	});
});
