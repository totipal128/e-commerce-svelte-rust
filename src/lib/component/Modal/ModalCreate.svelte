<script>
    import {createEventDispatcher, onMount, tick} from 'svelte';
    import {invoke} from "@tauri-apps/api/core";

    const dispatch = createEventDispatcher();


    let {
        loading = false,
        formatData = null,
        formatFormData = [
            {title: "format text", value: "data", type: "text", is_required: true, placeholder: ""},
            {title: "format textarea", value: "textarea", type: "text", is_required: true, placeholder: ""},
        ],
    } = $props()

    let dataSave = $state(structuredClone(formatData))

    function handlerClose() {
        dispatch("close", false);
    }

    let errors = $state({})
    let focusThis = {}

    function handlerSave(e) {
        e.preventDefault()

        let firstError = null

        for (let elm of formatFormData) {

            if (elm.is_required && (dataSave[elm.value] === "" || dataSave[elm.value] === null)) {
                errors[elm.value] = "Field ini wajib di isi";

                if (!firstError) {
                    firstError = elm.value
                }

            }
        }

        if (firstError) {
            tick().then((data) => {
                focusThis[firstError]?.focus();
            })

            return;
        }

        dispatch("create", dataSave)
        return;
    }
</script>

<div class="absolute bg-black w-screen h-screen z-11 left-0 top-0" style="opacity: 0.95">
</div>
<div class="absolute inset-0 flex items-center justify-center z-[11] " on:click={handlerClose}>
	<div class=" bg-white min-w-200  rounded-2xl" on:click|stopPropagation>
		<div class="relative flex items-center justify-center py-6 ">
			<span class="text-3xl font-semibold">Create Data</span>

			<button
					on:click={handlerClose}
					class="absolute right-4 top-4 hover:bg-gray-200 p-3 rounded-full"
			>
				X
			</button>
		</div>

		<form class="relative overflow-y-auto" on:submit={handlerSave}>
			<!-- CONTENT -->
			<div class="p-3 text-2xl">
				<div class="max-w-sm mx-auto space-y-4">
					{#each formatFormData as form, key}
						{#if form.type === "text"}
							<div>
								<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">{form.title} </label>
								<input
										bind:value={dataSave[form.value]}
										bind:this={focusThis[form.value]}
										type="text"
										id="visitors"
										class={
										 ` text-heading text-sm rounded-base block w-full px-3 py-2.5 shadow-xl
										 ${
                                             errors[form.value]
                                             ? "border-red-500 bg-red-50 focus:ring-red-200"
                                             : "border-gray-300 focus:border-blue-500 focus:ring-blue-200"
										 }
										   `
									}
										placeholder="{errors[form.value] ? errors[form.value] : form.placeholder}"
								/>
							</div>
						{:else if form.type === "textarea"}
							<div>
								<label for="visitors" class="block mb-2.5 text-sm font-medium text-heading">{form.title} </label>
								<textarea
										bind:value={dataSave[form.value]}
										bind:this={focusThis[form.value]}
										type="text"
										id="visitors"
										class={
										 ` text-heading text-sm rounded-base block w-full px-3 py-2.5 shadow-xl
										 ${
                                             errors[form.value]
                                             ? "border-red-500 bg-red-50 focus:ring-red-200"
                                             : "border-gray-300 focus:border-blue-500 focus:ring-blue-200"
										 }
										   `
									}
										placeholder="{errors[form.value] ? errors[form.value] : form.placeholder}"
								/>
							</div>
						{/if}

					{/each}
				</div>


			</div>

			<!-- FOOTER -->
			<div class=" p-4 flex justify-center gap-3">
				<button
						class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-800 text-white"
				>
					Simpan
				</button>
				<button
						on:click={handlerClose}
						class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300"
				>
					Close
				</button>
			</div>
		</form>
	</div>
</div>