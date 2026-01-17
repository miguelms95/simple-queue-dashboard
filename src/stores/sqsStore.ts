import { ref } from 'vue';

export const isConfigured = ref(false);

export function setConfigured(value: boolean) {
  isConfigured.value = value;
  if (value) {
    localStorage.setItem('sqs_configured', 'true');
  } else {
    localStorage.removeItem('sqs_configured');
  }
}

// Check if previously configured on app load
if (localStorage.getItem('sqs_configured') === 'true') {
  isConfigured.value = true;
}
