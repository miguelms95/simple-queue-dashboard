import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export const isConfigured = ref(false);
export const region = ref('eu-west-1');
export const endpoint = ref('http://localhost:4566');

export function setConfigured(value: boolean) {
  isConfigured.value = value;
  if (value) {
    localStorage.setItem('sqs_configured', 'true');
  } else {
    localStorage.removeItem('sqs_configured');
  }
}

export function setRegion(value: string) {
    region.value = value;
    localStorage.setItem('sqs_region', value);
}

export function setEndpoint(value: string) {
    endpoint.value = value;
    localStorage.setItem('sqs_endpoint', value);
}

export async function reconfigureSqs(newRegion: string) {
    setRegion(newRegion);
    try {
        await invoke('configure_sqs', { endpoint: endpoint.value, region: newRegion });
        setConfigured(true);
    } catch (e) {
        setConfigured(false);
        throw e;
    }
}

// Check if previously configured on app load
if (localStorage.getItem('sqs_configured') === 'true') {
  isConfigured.value = true;
}

const storedRegion = localStorage.getItem('sqs_region');
if (storedRegion) {
    region.value = storedRegion;
}

const storedEndpoint = localStorage.getItem('sqs_endpoint');
if (storedEndpoint) {
    endpoint.value = storedEndpoint;
}
