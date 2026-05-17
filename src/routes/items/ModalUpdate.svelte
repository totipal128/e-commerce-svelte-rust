<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let { open = false, id = 0 } = $props();

  let detailData = $state(null);

  let loading = $state(true);
  function createNewPriceItem() {
    return {
      barcode: "",
      type_unit: null,
      parent_type_unit: null,
      price_buy: 0,
      price_sell: 0,
      content: 0,
    };
  }
  /** @type {any} */
  let itemsData = $state({
    barcode: "",
    name: "",
    type_unit: null,
    jenis_barang_id: null,
    qty_stock: 0,
    price: [createNewPriceItem()],
  });
  let data = $state([]);

  async function saveData(data) {
    loading = true;
    try {
      const result = await invoke("items_update", {
        data: data,
      });

      data = result.results;
    } catch (err) {
      console.log("err", err);
    } finally {
      closeModal();
      loading = false;
    }
  }

  async function dataDetail() {
    loading = true;
    try {
      const result = await invoke("get_items_by_id", {
        id: id,
      });

      itemsData = result.data;

      console.log(itemsData);
    } catch (err) {
      console.log("err", err);
    } finally {
      loading = false;
    }
  }

  let listKategori = $state([]);
  let listSatuan = $state([]);

  onMount(async () => {
    dataDetail();
    try {
      const resKategori = await invoke("jenis_barang_list");
      if (resKategori.results) listKategori = resKategori.results;
      const resSatuan = await invoke("unit_barang_list");
      if (resSatuan.results) listSatuan = resSatuan.results;
    } catch (e) {
      console.error(e);
    }
  });

  function closeModal() {
    dispatch("close");
  }

  function removeRowPrice(i) {
    itemsData.price.splice(i, 1);
  }

  function addRowPrice() {
    itemsData.price.push(createNewPriceItem());
  }

  function save(e) {
    itemsData.price[0].barcode = itemsData.barcode;
    itemsData.price[0].type_unit = itemsData.type_unit;

    // Ensure all numeric fields are parsed correctly
    for (let p of itemsData.price) {
      p.price_buy = Number(p.price_buy) || 0;
      p.price_sell = Number(p.price_sell) || 0;
      p.content = Number(p.content) || 0;
    }
    itemsData.qty_stock = Number(itemsData.qty_stock) || 0;
    itemsData.jenis_barang_id = itemsData.jenis_barang_id
      ? Number(itemsData.jenis_barang_id)
      : null;

    saveData(itemsData);
  }
</script>

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black"
    style="opacity: 0.9"
    on:click={closeModal}
  ></div>
  <!-- ModalCreate -->
  <div
    class="absolute z-51 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
           bg-neutral-primary-soft border border-default
            rounded-base shadow-sm p-4 md:p-6"
    on:click|stopPropagation
  >
    <!-- Close button -->
    <button
      on:click={closeModal}
      class="absolute top-3 right-3 text-body hover:bg-neutral-tertiary rounded-base w-9 h-9 flex items-center justify-center"
    >
      ✕
    </button>

    <!-- Content -->
    <div class="relative h-full">
      <!--				<svg class="mx-auto mb-4 text-fg-disabled w-12 h-12" viewBox="0 0 24 24" fill="none">-->
      <!--					<path stroke="currentColor" stroke-width="2"-->
      <!--						  d="M12 13V8m0 8h.01M21 12a9 9 0 1 1-18 0Z"/>-->

      <!--				</svg>-->

      <div class="mx-auto mb-4 text-fg text-2xl text-center w-50 h-12">
        Update Data

        <div class="h-[2px] bg-gray-400 my-2"></div>
      </div>

      <form class="relative overflow-y-auto" on:submit|preventDefault={save}>
        <div class="max-w-sm mx-auto space-y-4">
          <div>
            <label
              for="visitors"
              class="block mb-2.5 text-sm font-medium text-heading"
              >Barcode</label
            >
            <input
              bind:value={itemsData.barcode}
              type="text"
              id="visitors"
              class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
              placeholder=""
              required
            />
          </div>
          <div>
            <label
              for="visitors"
              class="block mb-2.5 text-sm font-medium text-heading"
              >Nama Barang</label
            >
            <input
              bind:value={itemsData.name}
              type="text"
              id="visitors"
              class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
              placeholder=""
              required
            />
          </div>

          <label
            for="countries"
            class="block mb-2.5 text-sm font-medium text-heading"
            >Kategori</label
          >
          <select
            id="countries"
            bind:value={itemsData.jenis_barang_id}
            class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
          >
            <option selected value={null}>-- Pilih Kategori --</option>
            {#each listKategori as k}
              <option value={k.id}>{k.name}</option>
            {/each}
          </select>

          <label
            for="countries"
            class="block mb-2.5 text-sm font-medium text-heading"
            >Satuan Barang</label
          >
          <select
            bind:value={itemsData.type_unit}
            id="countries"
            class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
          >
            <option selected value={null}>-- Pilih Satuan --</option>
            {#each listSatuan as s}
              <option value={s.name}>{s.name}</option>
            {/each}
          </select>

          <div>
            <label
              for="visitors"
              class="block mb-2.5 text-sm font-medium text-heading">Stok</label
            >
            <input
              bind:value={itemsData.qty_stock}
              type="number"
              id="visitors"
              class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
              placeholder=""
              required
            />
          </div>
        </div>

        <div
          class="relative overflow-x-auto overflow-y-auto max-h-[40vh] bg-neutral-primary-soft shadow-xs rounded-base border border-default mt-10"
        >
          <table class="relative text-sm text-left rtl:text-right text-body">
            <thead
              class="sticky top-0 text-sm text-body bg-neutral-secondary-soft border-b rounded-base border-default"
            >
              <tr>
                <th scope="col" class="px-6 py-3 font-medium"> Barcode </th>
                <th scope="col" class="px-6 py-3 font-medium">
                  Satuan Barang
                </th>
                <th scope="col" class="px-6 py-3 font-medium">
                  Induk Satuan Barang
                </th>
                <th scope="col" class="px-6 py-3 font-medium"> Harga Beli </th>
                <th scope="col" class="px-6 py-3 font-medium"> Harga Jual </th>
                <th scope="col" class="px-6 py-3 font-medium"> Isi </th>
                <th scope="col" class="px-6 py-3 font-medium"> # </th>
              </tr>
            </thead>
            <tbody>
              {#each itemsData.price as price, index}
                <tr class="bg-neutral-primary border-b border-default">
                  <th
                    scope="row"
                    class="px-6 py-4 font-medium text-heading whitespace-nowrap"
                  >
                    {#if index == 0}
                      <input
                        bind:value={itemsData.barcode}
                        type="text"
                        id="visitors"
                        class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
                        placeholder=""
                        required
                      />
                    {:else}
                      <input
                        bind:value={itemsData.price[index].barcode}
                        type="text"
                        id="visitors"
                        class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
                        placeholder=""
                        required
                      />
                    {/if}
                  </th>
                  <td class="px-6 py-4">
                    {#if index == 0}
                      <select
                        bind:value={itemsData.type_unit}
                        id="countries"
                        class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
                      >
                        <option selected value={null}>-- Pilih Satuan --</option
                        >
                        {#each listSatuan as s}
                          <option value={s.name}>{s.name}</option>
                        {/each}
                      </select>
                    {:else}
                      <select
                        bind:value={itemsData.price[index].type_unit}
                        id="countries"
                        class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
                      >
                        <option selected value={null}>-- Pilih Satuan --</option
                        >
                        {#each listSatuan as s}
                          <option value={s.name}>{s.name}</option>
                        {/each}
                      </select>
                    {/if}
                  </td>
                  <td>
                    <select
                      bind:value={itemsData.price[index].parent_type_unit}
                      id="countries"
                      class="block w-full px-3 py-2.5 bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand shadow-xs placeholder:text-body"
                    >
                      <option selected value={null}
                        >-- Pilih Induk Satuan --</option
                      >
                      {#each listSatuan as s}
                        <option value={s.name}>{s.name}</option>
                      {/each}
                    </select>
                  </td>

                  <td class="px-6 py-4">
                    <input
                      bind:value={itemsData.price[index].price_buy}
                      type="number"
                      id="visitors"
                      class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
                      placeholder=""
                      required
                    />
                  </td>
                  <td class="px-6 py-4">
                    <input
                      bind:value={itemsData.price[index].price_sell}
                      type="number"
                      id="visitors"
                      class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
                      placeholder=""
                      required
                    />
                  </td>
                  <td class="px-6 py-4">
                    <input
                      bind:value={itemsData.price[index].content}
                      type="number"
                      id="visitors"
                      class="bg-neutral-secondary-medium border border-default-medium text-heading text-sm rounded-base focus:ring-brand focus:border-brand block w-full px-3 py-2.5 shadow-xs placeholder:text-body"
                      placeholder=""
                      required
                    />
                  </td>
                  <td>
                    {#if index != 0}
                      <button
                        on:click={(e) => removeRowPrice(index)}
                        type="button"
                        class="text-white bg-danger box-border border border-transparent hover:bg-danger-strong focus:ring-4 focus:ring-danger-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none"
                      >
                        <img
                          src="/icon/trash.svg"
                          class="h-6 w-6 p-1 justify-center"
                          alt="Tauri Logo"
                        />
                      </button>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
          <button
            on:click={addRowPrice}
            class="text-white bg-gray-400 box-border border border-transparent hover:bg-brand-strong focus:ring-4 mt-2 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none"
          >
            Tambah
          </button>
        </div>

        <div class="flex gap-4 justify-center mt-5">
          <button
            class="text-white bg-brand box-border border border-transparent hover:bg-brand-strong focus:ring-4 focus:ring-brand-medium shadow-xs font-medium leading-5 rounded-base text-sm px-4 py-2.5 focus:outline-none"
          >
            Simpan
          </button>

          <button
            on:click|stopPropagation={closeModal}
            class="bg-danger hover:bg-danger-strong text-white px-4 py-2.5 rounded-base"
          >
            Batal
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
