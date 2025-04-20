import { invoke } from '@tauri-apps/api/core';
import type { PageLoad } from './$types';
import type { Encounter } from '../../../types/Encounter';

export const load: PageLoad = async ({ params }) => {
  const id = parseInt(params.id);
  const encounter: Encounter = await invoke("get_encounter", { id });
  
  return {
    encounter
  };
};
