<script>
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { showToast } from "$lib/store/toast.js";

  const dispatch = createEventDispatcher();

  let { openRemove = false, id = 0 } = $props();

  let loading = $state(true);
  let data = $state({});

  async function fetchData() {
    loading = true;
    try {
      const result = await invoke("jenis_barang_by_id", {
        id: id,
      });
      if (result.data) {
        data = result.data;
      }
    } catch (err) {
      console.log("err", err);
      showToast("Gagal mengambil data kategori: " + err, "error");
    } finally {
      loading = false;
    }
  }

  async function deleteData() {
    loading = true;
    try {
      await invoke("jenis_barang_delete", {
        id: id,
      });
      showToast("Berhasil menghapus data kategori", "success");
    } catch (err) {
      console.log("err", err);
      showToast("Gagal menghapus data: " + err, "error");
    } finally {
      loading = false;
      closeModal(true);
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

  async function confirmModal() {
    await deleteData();
  }
</script>

{#if openRemove}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black"
    style="opacity: 0.9"
    on:click={closeModal}
  ></div>
  <!-- Modal Remove -->
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

    <div class="text-center mt-5">
      <svg
        class="mx-auto mb-4 text-fg-disabled w-12 h-12"
        viewBox="0 0 24 24"
        fill="none"
      >
        <path
          stroke="currentColor"
          stroke-width="2"
          d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0Z"
        />
      </svg>

      {#if loading && !data.name}
        <div class="flex justify-center p-5">
          <img src="/icon/gift/loading.gif" class="h-8 w-8" alt="Loading" />
        </div>
      {:else}
        <h3 class="mb-6 text-body">
          Apakah Anda Yakin Ingin Menghapus Data Kategori <strong
            >{data.name || ""}</strong
          > ?
        </h3>
      {/if}

      <div class="flex gap-4 justify-center">
        <button
          on:click={confirmModal}
          disabled={loading}
          class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base"
        >
          Ya, Hapus
        </button>

        <button
          on:click={closeModal}
          class="bg-neutral-secondary-medium hover:bg-neutral-tertiary-medium px-4 py-2.5 rounded-base"
        >
          Tidak
        </button>
      </div>
    </div>
  </div>
{/if}
