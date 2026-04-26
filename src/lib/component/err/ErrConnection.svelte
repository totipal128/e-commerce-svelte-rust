<script>
    import {createEventDispatcher, onMount, tick} from 'svelte';
    const dispatch = createEventDispatcher();

    let {
        desc="Connection Failed !!!"
    } = $props()

    function checkConnection(){
        dispatch("check");
    }
</script>

<div class="absolute bg-black h-full w-full text-center z-300" style="opacity: 0.50">
</div>
<div class="absolute h-full w-full text-center z-301">
    <div class="fixed inset-0 flex items-center justify-center bg-black/50">
        <div class="bg-white w-[500px] h-min-[300px] rounded-lg shadow-lg p-6">

            <div class="flex justify-center">
                <svg
                        xmlns="http://www.w3.org/2000/svg"
                        id="Isolation_Mode"
                        data-name="Isolation Mode"
                        viewBox="0 0 24 24"
                        width="150"
                        height="150"
                        class="text-red-500"
                        fill="currentColor"
                >
                    <path
                            d="M12,24A12,12,0,1,1,24,12,12.013,12.013,0,0,1,12,24ZM12,3a9,9,0,1,0,9,9A9.01,9.01,0,0,0,12,3Z"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            fill="red"
                    />
                    <rect x="10.5" y="5.318" width="3" height="8.836"/><rect x="10.565" y="15.994" width="3" height="3"/>
                </svg>
            </div>

            <p class="text-center mt-5 text-2xl">{desc}</p>

            <button type="button"
                    on:click={checkConnection}
                    class=" p-3 mt-10 mb-5 text-body bg-neutral-secondary-medium box-border border border-default-medium hover:bg-neutral-tertiary-medium hover:text-heading focus:ring-4 focus:ring-neutral-tertiary shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none">
                <div class="flex">
                    <img src="/icon/refresh.svg" class="w-5 h-5 pr-2" alt="back"/> Check
                </div>
            </button>

        </div>
    </div>
</div>