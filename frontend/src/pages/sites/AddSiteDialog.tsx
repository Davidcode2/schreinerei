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
import { useBillingSettings, useCreateSite } from "@/lib/api/hooks";
import type { InvoicePricingMode, ProjectType } from "@/types/sites";

type PricingModeValue = InvoicePricingMode | "none";

function formatMoney(value: number | null): string {
	if (value == null) return "";
	return (value / 100).toFixed(2);
}

function parseMoney(value: string): number | null {
	const normalized = value.replace(",", ".").trim();
	if (!normalized) return null;
	const parsed = Number(normalized);
	if (!Number.isFinite(parsed) || parsed < 0) return null;
	return Math.round(parsed * 100);
}

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
	const [pricingMode, setPricingMode] = useState<PricingModeValue>("none");
	const [hourlyRate, setHourlyRate] = useState("");
	const [fixedPrice, setFixedPrice] = useState("");

	const createSite = useCreateSite();
	const { data: billingSettings } = useBillingSettings();

	const defaultHourlyRate = billingSettings?.default_hourly_rate_cents ?? null;

	const resetForm = () => {
		setName("");
		setProjectType("external_site");
		setCustomerName("");
		setLocation("");
		setDescription("");
		setStartDate("");
		setEndDate("");
		setEstimatedDays("");
		if (defaultHourlyRate != null) {
			setPricingMode("hourly_rate");
			setHourlyRate(formatMoney(defaultHourlyRate));
		} else {
			setPricingMode("none");
			setHourlyRate("");
		}
		setFixedPrice("");
	};

	const handleOpenChange = (open: boolean) => {
		if (!open) {
			resetForm();
		}
		onOpenChange(open);
	};

	const customerRequired = projectType === "external_site";
	const parsedHourlyRate = parseMoney(hourlyRate);
	const parsedFixedPrice = parseMoney(fixedPrice);
	const pricingIsValid =
		pricingMode === "none" ||
		(pricingMode === "hourly_rate" && parsedHourlyRate != null) ||
		(pricingMode === "fixed_price" && parsedFixedPrice != null);
	const isFormValid = Boolean(name && (!customerRequired || customerName) && pricingIsValid);

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
			invoice_pricing_mode?: InvoicePricingMode;
			hourly_rate_cents?: number;
			fixed_price_cents?: number;
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
		if (pricingMode === "hourly_rate" && parsedHourlyRate != null) {
			payload.invoice_pricing_mode = "hourly_rate";
			payload.hourly_rate_cents = parsedHourlyRate;
		}
		if (pricingMode === "fixed_price" && parsedFixedPrice != null) {
			payload.invoice_pricing_mode = "fixed_price";
			payload.fixed_price_cents = parsedFixedPrice;
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
			<DialogContent className="flex max-h-[90vh] flex-col overflow-hidden sm:max-w-md">
				<DialogHeader>
					<DialogTitle className="flex items-center gap-2.5 font-display">
						<div className="flex h-9 w-9 items-center justify-center rounded-lg bg-accent">
							<Building2 className="h-4 w-4 text-muted-foreground" />
						</div>
						Projekt anlegen
					</DialogTitle>
					<DialogDescription>Externes oder internes Projekt anlegen</DialogDescription>
				</DialogHeader>

				<div className="min-h-0 space-y-5 overflow-y-auto py-4 pr-1">
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

					<div className="space-y-4 rounded-lg border border-border/70 bg-card/70 p-4 shadow-sm">
						<p className="text-sm font-medium">Abrechnung</p>

						<div className="space-y-2">
							<Label htmlFor="pricingMode">Rechnungslogik</Label>
							<Select value={pricingMode} onValueChange={(value) => setPricingMode(value as PricingModeValue)}>
								<SelectTrigger id="pricingMode" className="h-10">
									<SelectValue placeholder="Rechnungslogik wählen" />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="none">Keine Vorgabe</SelectItem>
									<SelectItem value="hourly_rate">Stundensatz</SelectItem>
									<SelectItem value="fixed_price">Pauschalpreis</SelectItem>
								</SelectContent>
							</Select>
						</div>

						{pricingMode === "hourly_rate" && (
							<div className="space-y-2">
								<Label htmlFor="hourlyRate">Stundensatz (EUR)</Label>
								<Input
									id="hourlyRate"
									type="number"
									min="0"
									step="0.01"
									inputMode="decimal"
									placeholder="z.B. 85,00"
									value={hourlyRate}
									onChange={(e) => setHourlyRate(e.target.value)}
									className="h-10"
								/>
							</div>
						)}

						{pricingMode === "fixed_price" && (
							<div className="space-y-2">
								<Label htmlFor="fixedPrice">Pauschalpreis (EUR)</Label>
								<Input
									id="fixedPrice"
									type="number"
									min="0"
									step="0.01"
									inputMode="decimal"
									placeholder="z.B. 2500,00"
									value={fixedPrice}
									onChange={(e) => setFixedPrice(e.target.value)}
									className="h-10"
								/>
							</div>
						)}
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
