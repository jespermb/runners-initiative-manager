import { invoke } from "@tauri-apps/api/core";
import type { Combatten } from "../../types/Combatten";
import type { Encounter } from "../../types/Encounter";

export function createCampaignDataManager(campaignId: number, initialEncounters: Encounter[] = [], initialCombattens: Combatten[] = []) {
    async function getCombattens(): Promise<Combatten[]> {
        return await invoke("get_all_combattens", { campaignId });
    }
    
    async function getEncounters(): Promise<Encounter[]> {
        return await invoke("get_all_encounters", { campaignId });
    }

    async function addCombatten(name: string): Promise<void> {
        await invoke("add_combatten", {
            name,
            campaignId,
        });
    }

    return {
        // Actions
        getCombattens,
        getEncounters,
        addCombatten
    };
}