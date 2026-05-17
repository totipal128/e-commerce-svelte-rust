<script>
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { showToast } from "$lib/store/toast.js";

  const dispatch = createEventDispatcher();

  let { open = false, id = 0 } = $props();

  let loading = $state(true);
  let jenisData = $state({
    id: 0,
    name: "",
    description: "",
  });

  async function fetchData() {
    loading = true;
    try {
      const result = await invoke("jenis_barang_by_id", {
        id: id,
      });
      if (result.data) {
        jenisData = { ...result.data };
      }
    } catch (err) {
      console.log("err", err);
      showToast("Gagal mengambil data kategori: " + err, "error");
    } finally {
      loading = false;
    }
  }

  async function updateData() {
    loading = true;
    try {
      const result = await invoke("jenis_barang_update", {
        data: jenisData,
      });
      showToast("Berhasil memperbarui data kategori", "success");
      closeModal();
    } catch (err) {
      console.log("err", err);
      showToast("Gagal memperbarui data: " + err, "error");
    } finally {
      loading = false;
    }
  }

  onMount(fetchData);

  function closeModal(confirm) {
    let c = false;
    if (typeof confirm !== "boolean") {
      c = true;
    }
    dispatch("close", { confirm: c });
  }

  function save(e) {
    updateData();
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black"
    style="opacity: 0.9"
    on:click={closeModal}
  ></div>
  <div
    class="absolute z-51 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
           bg-neutral-primary-soft border border-default
            rounded-base shadow-sm p-4 md:p-6"
    on:click|stopPropagation
  >
    <button
      on:click={closeModal}
      class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center"
    >
      ✕
    </button>

    <div class="relative h-full">
      <div
        class="mx-auto mb-4 text-fg text-lg md:text-2xl text-center w-full min-h-12"
      >
        Ubah Kategori Barang
        <div class="h-[2px] bg-gray-400 my-2"></div>
      </div>

      {#if loading}
        <div class="flex justify-center p-5">
          <img src="/icon/gift/loading.gif" class="h-10 w-10" alt="Loading" />
        </div>
      {:else}
        <form class="relative mt-5" on:submit|preventDefault={save}>
          <div class="grid grid-cols-2 gap-4 place-items-stretch">
            <div class="col-span-2">
              <label class="block mb-2 text-sm font-medium text-heading"
                >Nama Kategori</label
              >
              <input
                bind:value={jenisData.name}
                type="text"
                class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
                placeholder="Masukkan nama kategori..."
                required
              />
            </div>
            <div class="col-span-2">
              <label class="block mb-2 text-sm font-medium text-heading"
                >Deskripsi (Opsional)</label
              >
              <textarea
                bind:value={jenisData.description}
                rows="2"
                class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
                placeholder="Deskripsi kategori..."
              ></textarea>
            </div>
          </div>

          <div class="flex gap-4 justify-center mt-8">
            <button
              disabled={loading}
              class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none"
            >
              Ubah Data
            </button>
            <button
              on:click={closeModal}
              type="button"
              class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base"
            >
              Batal
            </button>
          </div>
        </form>
      {/if}
    </div>
  </div>
{/if}
