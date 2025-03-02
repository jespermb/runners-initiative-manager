import { invoke } from '@tauri-apps/api/core';
import type { PageLoad } from './$types';
import type { Campaign } from '../../../types/Campaign';
import type { Combatten } from '../../../types/Combatten';
import type { Encounter } from '../../../types/Encounter';

export const load: PageLoad = async ({ params }) => {
  const id = parseInt(params.id);
  const campaign: Campaign = await invoke("get_campaign", { id });

  const combattens: Combatten[] = await invoke("get_all_combattens", { campaignId: id });
  const encounters: Encounter[] = await invoke("get_all_encounters", { campaignId: id });
	return {
		campaign,
    combattens,
    encounters,
	};
};