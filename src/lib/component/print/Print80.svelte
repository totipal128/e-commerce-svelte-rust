<script>
    function printReceipt() {
        window.print();
    }
</script>

<div class="receipt">
	<h3>TOKO MAJU</h3>
	<p>--------------------</p>
	<p>Item A 2 x 5000</p>
	<p>Item B 1 x 3000</p>
	<p>--------------------</p>
	<p>Total: Rp 13.000</p>
</div>

<button on:click={printReceipt}>
	Print
</button>

<style>
    @media print {
        @page {
            size: 80mm auto;
            margin: 0;
        }

        body {
            margin: 0;
            padding: 0;
            width: 80mm;
        }

        .receipt {
            width: 80mm;
            font-family: monospace;
            font-size: 12px;
        }
    }

</style>