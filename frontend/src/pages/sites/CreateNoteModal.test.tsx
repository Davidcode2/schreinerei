import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@/test/utils";
import { CreateNoteModal } from "./CreateNoteModal";

const composerPropsSpy = vi.fn();

vi.mock("./ProjectTimelineComposer", () => ({
	ProjectTimelineComposer: (props: unknown) => {
		composerPropsSpy(props);
		return <div data-testid="project-timeline-composer" />;
	},
}));

describe("CreateNoteModal", () => {
	it("renders the shared timeline composer with the stable note entrypoint contract", () => {
		const onOpenChange = vi.fn();
		const onSuccess = vi.fn();

		render(
			<CreateNoteModal
				open
				onOpenChange={onOpenChange}
				siteId="site-1"
				onSuccess={onSuccess}
			/>,
		);

		expect(screen.getByTestId("project-timeline-composer")).toBeInTheDocument();
		expect(composerPropsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				open: true,
				onOpenChange,
				siteId: "site-1",
				onSuccess,
				cameraMode: false,
			}),
		);
	});
});
