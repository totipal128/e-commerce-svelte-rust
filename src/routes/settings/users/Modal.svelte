<script>
    import {createEventDispatcher} from 'svelte';

    const dispatch = createEventDispatcher();

    let {open = false} = $props();

    function closeModal() {
        dispatch('close');
    }
</script>

{#if open}
    <!-- Overlay -->
    <div
            class="fixed inset-0 z-50 flex items-center justify-center
           bg-black/40"
            on:click={closeModal}
    >
        <!-- Modal -->
        <div
                class="relative bg-neutral-primary-soft border border-default
             rounded-base shadow-sm p-4 md:p-6 w-full max-w-md"
                on:click|stopPropagation
        >
            <!-- Close button -->
            <button
                    on:click={closeModal}
                    class="absolute top-3 right-3 text-body
               hover:bg-neutral-tertiary rounded-base
               w-9 h-9 flex items-center justify-center">
                ✕
            </button>

            <!-- Content -->
            <div class="text-center">
                <svg class="mx-auto mb-4 text-fg-disabled w-12 h-12" viewBox="0 0 24 24" fill="none">
                    <path stroke="currentColor" stroke-width="2"
                          d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0Z"/>
                </svg>

                <h3 class="mb-6 text-body">
                    Are you sure you want to delete this product from your account?
                </h3>

                <div class="flex gap-4 justify-center">
                    <button
                            on:click={closeModal}
                            class="bg-danger hover:bg-danger-strong
                   text-white px-4 py-2.5 rounded-base">
                        Yes, I'm sure
                    </button>

                    <button
                            on:click={closeModal}
                            class="bg-neutral-secondary-medium
                   hover:bg-neutral-tertiary-medium
                   px-4 py-2.5 rounded-base">
                        No, cancel
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}
