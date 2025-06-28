import { invoke } from "@tauri-apps/api/core";
import type { Combatten } from "../../types/Combatten";
import { z } from "zod";

// Validation schemas
const newCombatantSchema = z.object({
    name: z.string().min(2, { message: "Name must be at least 2 characters long" }),
    physical: z.number().int().min(1, { message: "Physical health must be a positive number" }),
    stun: z.number().int().min(1, { message: "Stun health must be a positive number" }),
});

const existingCombatantSchema = z.object({
    selectedCombattenId: z.number({ 
        required_error: "Please select a combatant",
        invalid_type_error: "Please select a combatant" 
    }),
});

const initiativeSchema = z.object({
    initiative: z.number().int().min(0, { message: "Initiative must be a non-negative number" }),
});

type ValidationErrors = {
    name?: string[] | undefined;
    physical?: string[] | undefined;
    stun?: string[] | undefined;
    initiative?: string[] | undefined;
    selectedCombattenId?: string[] | undefined;
};

export function useCombattentForm() {
    async function validateNewCombatant(name: string, physical: number, stun: number): Promise<{ valid: boolean; errors: ValidationErrors }> {
        const validationResult = newCombatantSchema.safeParse({ 
            name, 
            physical: parseInt(`${physical}`), 
            stun: parseInt(`${stun}`) 
        });

        if (!validationResult.success) {
            return {
                valid: false,
                errors: validationResult.error.flatten().fieldErrors
            };
        }

        return { valid: true, errors: {} };
    }

    async function validateExistingCombatant(selectedCombattenId: number | null): Promise<{ valid: boolean; errors: ValidationErrors }> {
        const validationResult = existingCombatantSchema.safeParse({ 
            selectedCombattenId
        });

        if (!validationResult.success) {
            return {
                valid: false,
                errors: validationResult.error.flatten().fieldErrors
            };
        }

        return { valid: true, errors: {} };
    }

    async function validateInitiative(initiative: number): Promise<{ valid: boolean; errors: ValidationErrors }> {
        const validationResult = initiativeSchema.safeParse({ 
            initiative: parseInt(`${initiative}`)
        });

        if (!validationResult.success) {
            return {
                valid: false,
                errors: validationResult.error.flatten().fieldErrors
            };
        }

        return { valid: true, errors: {} };
    }

    async function createNewCombatant(
        name: string, 
        combatantType: string, 
        physical: number, 
        stun: number, 
        campaignId: number
    ): Promise<{ success: boolean; combatten?: Combatten; error?: string }> {
        try {
            const combatten: Combatten = await invoke("add_combatten", { 
                name, 
                combattenType: combatantType,
                physical: parseInt(`${physical}`), 
                stun: parseInt(`${stun}`), 
                campaignId: campaignId 
            });
            
            return { success: true, combatten };
        } catch (error) {
            return { success: false, error: `${error}` };
        }
    }

    async function addCombattentToEncounter(
        encounterId: number,
        combattenId: number,
        initiative: number
    ): Promise<{ success: boolean; message?: string; error?: string }> {
        try {
            const message = await invoke("add_combatten_to_encounter", { 
                encounterId: encounterId, 
                initiative: parseInt(`${initiative}`), 
                combattenId: combattenId 
            });
            
            return { success: true, message: `${message}` };
        } catch (error) {
            return { success: false, error: `${error}` };
        }
    }

    async function loadCombattens(campaignId: number): Promise<{ success: boolean; combattens?: Combatten[]; error?: string }> {
        try {
            const combattens = await invoke("get_all_campaign_combattens", { campaignId });
            return { success: true, combattens };
        } catch (error) {
            return { success: false, error: `Error loading combattens: ${error}` };
        }
    }

    function filterCombattens(
        combattens: Combatten[], 
        searchQuery: string, 
        existingCombattens: number[]
    ): Combatten[] {
        // First filter out only PCs that are already in the encounter (allow NPCs to be re-added)
        const availableCombattens = combattens.filter(c => 
            !(existingCombattens.includes(c.id) && c.combatten_type === 'pc')
        );
        
        // Then filter by search query
        if (searchQuery.trim() === "") {
            return availableCombattens;
        } else {
            const query = searchQuery.toLowerCase();
            return availableCombattens.filter(c => 
                c.name.toLowerCase().includes(query)
            );
        }
    }

    function isStep1Valid(addType: string, name: string, physical: number, stun: number, selectedCombattenId: number | null): boolean {
        if (addType === "new") {
            return name.trim().length >= 2 && physical > 0 && stun > 0;
        } else {
            return selectedCombattenId !== null;
        }
    }

    return {
        validateNewCombatant,
        validateExistingCombatant,
        validateInitiative,
        createNewCombatant,
        addCombattentToEncounter,
        loadCombattens,
        filterCombattens,
        isStep1Valid
    };
}