import { ref } from 'vue';

export const isConfigured = ref(false);
export const region = ref('eu-west-1');

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

// Check if previously configured on app load
if (localStorage.getItem('sqs_configured') === 'true') {
  isConfigured.value = true;
}

const storedRegion = localStorage.getItem('sqs_region');
if (storedRegion) {
    region.value = storedRegion;
}
