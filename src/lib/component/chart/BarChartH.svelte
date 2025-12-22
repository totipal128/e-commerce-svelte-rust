<script>
    import {onMount, onDestroy} from "svelte";
    import ApexCharts from "apexcharts";

    let chartEl;
    let chart;

    // ambil warna dari CSS variable
    const getBrandColor = () => {
        const style = getComputedStyle(document.documentElement);
        return style.getPropertyValue("--color-fg-brand").trim() || "#1447E6";
    };

    const getBrandSecondaryColor = () => {
        const style = getComputedStyle(document.documentElement);
        return style.getPropertyValue("--color-fg-brand-subtle").trim() || "#8EA2FF";
    };

    onMount(() => {
        const brandColor = getBrandColor();
        const brandSecondaryColor = getBrandSecondaryColor();

        const options = {
            colors: [brandColor, brandSecondaryColor],
            series: [
                {
                    name: "Organic",
                    data: [
                        {x: "Mon", y: 231},
                        {x: "Tue", y: 122},
                        {x: "Wed", y: 63},
                        {x: "Thu", y: 421},
                        {x: "Fri", y: 122},
                        {x: "Sat", y: 323},
                        {x: "Sun", y: 111}
                    ]
                },
                {
                    name: "Social media",
                    data: [
                        {x: "Mon", y: 232},
                        {x: "Tue", y: 113},
                        {x: "Wed", y: 341},
                        {x: "Thu", y: 224},
                        {x: "Fri", y: 522},
                        {x: "Sat", y: 411},
                        {x: "Sun", y: 243}
                    ]
                }
            ],
            chart: {
                type: "bar",
                height: 320,
                fontFamily: "Inter, sans-serif",
                toolbar: {show: false}
            },
            plotOptions: {
                bar: {
                    columnWidth: "70%",
                    borderRadius: 8,
                    borderRadiusApplication: "end"
                }
            },
            tooltip: {
                shared: true,
                intersect: false
            },
            grid: {show: false},
            dataLabels: {enabled: false},
            legend: {show: false},
            xaxis: {
                labels: {
                    style: {
                        cssClass: "text-xs font-normal fill-body"
                    }
                },
                axisBorder: {show: false},
                axisTicks: {show: false}
            },
            yaxis: {show: false},
            fill: {opacity: 1}
        };

        chart = new ApexCharts(chartEl, options);
        chart.render();
    });

    onDestroy(() => {
        chart?.destroy();
    });
</script>

<div class="max-w-sm w-full bg-neutral-primary-soft border border-default rounded-base shadow-xs p-4 md:p-6">
    <div class="flex justify-between pb-4 mb-4 border-b border-light">
        <div class="flex items-center">
            <div class="w-12 h-12 bg-neutral-primary-medium border border-default-medium flex items-center justify-center rounded-full me-3">
                <!-- icon -->
            </div>
            <div>
                <h5 class="text-2xl font-semibold text-heading">3.4k</h5>
                <p class="text-sm text-body">Leads generated per week</p>
            </div>
        </div>
        <span class="text-xs font-medium text-fg-success">▲ 42.5%</span>
    </div>

    <div class="grid grid-cols-2 mb-4">
        <div class="text-sm">
            <span class="text-body">Money spent:</span>
            <span class="font-semibold">$3,232</span>
        </div>
        <div class="text-sm text-right">
            <span class="text-body">Conversion:</span>
            <span class="font-semibold">1.2%</span>
        </div>
    </div>

    <!-- Chart -->
    <div bind:this={chartEl}></div>

    <div class="flex justify-between items-center border-t border-light pt-4 mt-4">
        <button class="text-sm text-body hover:text-heading">
            Last 7 days
        </button>

        <a href="#" class="text-sm font-medium text-fg-brand">
            Leads Report →
        </a>
    </div>
</div>
