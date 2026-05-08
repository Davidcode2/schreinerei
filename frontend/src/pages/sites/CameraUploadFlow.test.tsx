import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@/test/utils";
import { CameraUploadFlow } from "./CameraUploadFlow";

const composerPropsSpy = vi.fn();

vi.mock("./ProjectTimelineComposer", () => ({
	ProjectTimelineComposer: (props: unknown) => {
		composerPropsSpy(props);
		return <div data-testid="project-timeline-composer" />;
	},
}));

describe("CameraUploadFlow", () => {
	it("routes the camera shortcut through the shared composer contract", () => {
		const onOpenChange = vi.fn();
		const onSuccess = vi.fn();

		render(
			<CameraUploadFlow
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
				cameraMode: true,
			}),
		);
	});
});
