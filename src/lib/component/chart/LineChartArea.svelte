<script>
    import {onMount, onDestroy} from "svelte";
    import ApexCharts from "apexcharts";

    let chartEl;
    let chart;

    const getBrandColor = () => {
        const computedStyle = getComputedStyle(document.documentElement);
        return computedStyle.getPropertyValue("--color-fg-brand").trim() || "#1447E6";
    };

    onMount(() => {
        const brandColor = getBrandColor();

        const options = {
            chart: {
                height: 180,
                width: "100%",
                type: "area",
                fontFamily: "Inter, sans-serif",
                toolbar: {show: false},
                dropShadow: {enabled: false}
            },
            tooltip: {
                enabled: true,
                x: {show: false}
            },
            fill: {
                type: "gradient",
                gradient: {
                    opacityFrom: 0.55,
                    opacityTo: 0,
                    shade: brandColor,
                    gradientToColors: [brandColor]
                }
            },
            dataLabels: {enabled: false},
            stroke: {width: 4},
            grid: {
                show: false,
                padding: {left: 2, right: 2, top: 0}
            },
            series: [
                {
                    name: "New users",
                    data: [6500, 6418, 6456, 6526, 6356, 6456],
                    color: brandColor
                }
            ],
            xaxis: {
                categories: [
                    "01 Feb",
                    "02 Feb",
                    "03 Feb",
                    "04 Feb",
                    "05 Feb",
                    "06 Feb",
                    "07 Feb"
                ],
                labels: {show: false},
                axisBorder: {show: false},
                axisTicks: {show: false}
            },
            yaxis: {show: false}
        };

        chart = new ApexCharts(chartEl, options);
        chart.render();
    });

    onDestroy(() => {
        chart?.destroy();
    });
</script>


<div class="max-w-sm w-full bg-neutral-primary-soft border border-default rounded-base shadow-xs p-4 md:p-6">
    <div class="flex justify-between items-start">
        <div>
            <h5 class="text-2xl font-semibold text-heading">32.4k</h5>
            <p class="text-body">Users this week</p>
        </div>

        <div class="flex items-center px-2.5 py-0.5 font-medium text-fg-success">
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-width="2"
                      d="M12 6v13m0-13 4 4m-4-4-4 4"/>
            </svg>
            12%
        </div>
    </div>

    <!-- Chart -->
    <div bind:this={chartEl} class="mt-4"></div>

    <div class="border-t border-light pt-4 md:pt-6 flex justify-between items-center">
        <button
                class="text-sm font-medium text-body hover:text-heading inline-flex items-center">
            Last 7 days
            <svg class="w-4 h-4 ms-1.5" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-width="2"
                      d="m19 9-7 7-7-7"/>
            </svg>
        </button>

        <a href="#"
           class="text-fg-brand text-sm font-medium inline-flex items-center">
            Users Report
            <svg class="w-4 h-4 ms-1.5" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-width="2"
                      d="M19 12H5m14 0-4 4m4-4-4-4"/>
            </svg>
        </a>
    </div>
</div>
