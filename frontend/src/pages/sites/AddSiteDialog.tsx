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
import { Textarea } from "@/components/ui/textarea";
import { useCreateSite } from "@/lib/api/hooks";

interface AddSiteDialogProps {
	open: boolean;
	onOpenChange: (open: boolean) => void;
}

export function AddSiteDialog({ open, onOpenChange }: AddSiteDialogProps) {
	const [name, setName] = useState("");
	const [customerName, setCustomerName] = useState("");
	const [location, setLocation] = useState("");
	const [description, setDescription] = useState("");

	const createSite = useCreateSite();

	const resetForm = () => {
		setName("");
		setCustomerName("");
		setLocation("");
		setDescription("");
	};

	const handleOpenChange = (open: boolean) => {
		if (!open) {
			resetForm();
		}
		onOpenChange(open);
	};

	const isFormValid = name && customerName;

	const handleSubmit = () => {
		if (!isFormValid) return;

		const payload: {
			name: string;
			customer_name: string;
			location?: string;
			description?: string;
		} = {
			name,
			customer_name: customerName,
		};

		if (location) {
			payload.location = location;
		}
		if (description) {
			payload.description = description;
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
						Baustelle anlegen
					</DialogTitle>
					<DialogDescription>Neue Baustelle erstellen</DialogDescription>
				</DialogHeader>

				<div className="space-y-5 py-4">
					<div className="space-y-2">
						<Label htmlFor="name">Baustellenname *</Label>
						<Input
							id="name"
							placeholder="z.B. Villa Müller"
							value={name}
							onChange={(e) => setName(e.target.value)}
							className="h-10"
						/>
					</div>

					<div className="space-y-2">
						<Label htmlFor="customerName">Kunde *</Label>
						<Input
							id="customerName"
							placeholder="z.B. Familie Müller"
							value={customerName}
							onChange={(e) => setCustomerName(e.target.value)}
							className="h-10"
						/>
					</div>

					<div className="space-y-2">
						<Label htmlFor="location">Standort</Label>
						<Input
							id="location"
							placeholder="z.B. Musterstraße 1, 12345 Berlin"
							value={location}
							onChange={(e) => setLocation(e.target.value)}
							className="h-10"
						/>
					</div>

					<div className="space-y-2">
						<Label htmlFor="description">Beschreibung</Label>
						<Textarea
							id="description"
							placeholder="z.B. Küchenumbau, neues Treppenhaus"
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
						{createSite.isPending ? "Wird erstellt..." : "Erstellen"}
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	);
}
