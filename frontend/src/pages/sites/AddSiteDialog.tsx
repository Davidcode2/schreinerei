import { Building2 } from "lucide-react";
import { useState } from "react";
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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { useCreateSite } from "@/lib/api/hooks";
import type { ProjectType } from "@/types/sites";

interface AddSiteDialogProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
}

export function AddSiteDialog({ open, onOpenChange }: AddSiteDialogProps) {
	const [name, setName] = useState("");
	const [projectType, setProjectType] = useState<ProjectType>("external_site");
	const [customerName, setCustomerName] = useState("");
	const [location, setLocation] = useState("");
	const [description, setDescription] = useState("");
	const [startDate, setStartDate] = useState("");
	const [endDate, setEndDate] = useState("");
	const [estimatedDays, setEstimatedDays] = useState("");

	const createSite = useCreateSite();

	const resetForm = () => {
		setName("");
		setProjectType("external_site");
		setCustomerName("");
		setLocation("");
		setDescription("");
		setStartDate("");
		setEndDate("");
		setEstimatedDays("");
	};

	const handleOpenChange = (open: boolean) => {
		if (!open) {
			resetForm();
		}
		onOpenChange(open);
	};

	const customerRequired = projectType === "external_site";
	const isFormValid = Boolean(name && (!customerRequired || customerName));

	const handleSubmit = () => {
		if (!isFormValid) return;

		const payload: {
			project_type: ProjectType;
			name: string;
			customer_name: string;
			location?: string;
			description?: string;
			start_date?: string;
			end_date?: string;
			estimated_days?: number;
		} = {
			project_type: projectType,
			name,
			customer_name: customerName,
		};

		if (location) {
			payload.location = location;
		}
		if (description) {
			payload.description = description;
		}
		if (startDate) {
			payload.start_date = startDate;
		}
		if (endDate) {
			payload.end_date = endDate;
		}
		if (estimatedDays) {
			payload.estimated_days = Number(estimatedDays);
		}

		createSite.mutate(payload, {
			onSuccess: () => {
				toast.success("Baustelle erstellt");
				handleOpenChange(false);
			},
			onError: (error) => {
				toast.error("Baustelle konnte nicht erstellt werden");
				console.error("Create site error:", error);
			},
		});
	};

	return (
		<Dialog open={open} onOpenChange={handleOpenChange}>
			<DialogContent className="sm:max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 font-display">
						<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
							<Building2 className="h-4 w-4 text-muted-foreground" />
						</div>
						Projekt anlegen
					</DialogTitle>
					<DialogDescription>Externes oder internes Projekt anlegen</DialogDescription>
				</DialogHeader>

				<div className="space-y-5 py-4">
					<div className="space-y-2">
						<Label htmlFor="projectType">Projektart</Label>
						<Select value={projectType} onValueChange={(value) => setProjectType(value as ProjectType)}>
							<SelectTrigger id="projectType" className="h-10">
								<SelectValue placeholder="Projektart wählen" />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="external_site">Baustelle extern</SelectItem>
								<SelectItem value="internal_workshop">Werkstatt intern</SelectItem>
							</SelectContent>
						</Select>
					</div>

					<div className="space-y-2">
						<Label htmlFor="name">Projektname *</Label>
						<Input
							id="name"
							placeholder={projectType === "external_site" ? "z.B. Villa Müller" : "z.B. Küchenvorbereitung"}
							value={name}
							onChange={(e) => setName(e.target.value)}
							className="h-10"
						/>
					</div>

					<div className="space-y-2">
						<Label htmlFor="customerName">{customerRequired ? "Kunde *" : "Kunde / Bezug"}</Label>
						<Input
							id="customerName"
							placeholder={customerRequired ? "z.B. Familie Müller" : "Optional für interne Projekte"}
							value={customerName}
							onChange={(e) => setCustomerName(e.target.value)}
							className="h-10"
						/>
					</div>

					<div className="space-y-2">
						<Label htmlFor="location">Standort</Label>
						<Input
							id="location"
							placeholder={projectType === "external_site" ? "z.B. Musterstraße 1, 12345 Berlin" : "z.B. Werkstatt, CNC-Bereich"}
							value={location}
							onChange={(e) => setLocation(e.target.value)}
							className="h-10"
						/>
					</div>

					<div className="grid gap-4 sm:grid-cols-2">
						<div className="space-y-2">
							<Label htmlFor="startDate">Start</Label>
							<Input id="startDate" type="date" value={startDate} onChange={(e) => setStartDate(e.target.value)} className="h-10" />
						</div>
						<div className="space-y-2">
							<Label htmlFor="endDate">Ende</Label>
							<Input id="endDate" type="date" value={endDate} onChange={(e) => setEndDate(e.target.value)} className="h-10" />
						</div>
					</div>

					<div className="space-y-2">
						<Label htmlFor="estimatedDays">Geplante Tage</Label>
						<Input id="estimatedDays" type="number" min="0" value={estimatedDays} onChange={(e) => setEstimatedDays(e.target.value)} className="h-10" />
					</div>

					<div className="space-y-2">
						<Label htmlFor="description">Planungsnotiz</Label>
						<Textarea
							id="description"
							placeholder="z.B. Küchenumbau, neues Treppenhaus, Werkstattvorbereitung"
							value={description}
							onChange={(e) => setDescription(e.target.value)}
							rows={3}
						/>
					</div>
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
						disabled={!isFormValid || createSite.isPending}
					>
							{createSite.isPending ? "Wird erstellt..." : "Projekt erstellen"}
						</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
