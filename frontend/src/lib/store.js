import { writable } from 'svelte/store';

export const diagramStore = writable(`graph TD
    A[Start] --> B{Is it?};
    B -- Yes --> C[OK];
    C --> D[End];
    B -- No --> E[Find Out];
    E --> B;
`);
