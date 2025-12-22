<script>
    import {onMount, onDestroy} from "svelte";
    import ApexCharts from "apexcharts";

    let chartEl;
    let chart;

    // helper ambil CSS variable
    const cssVar = (name, fallback) => {
        const style = getComputedStyle(document.documentElement);
        return style.getPropertyValue(name).trim() || fallback;
    };

    onMount(() => {
        const brandColor = cssVar("--color-fg-brand", "#1447E6");
        const brandSecondaryColor = cssVar("--color-fg-brand-subtle", "#6B7CFF");
        const brandTertiaryColor = cssVar("--color-fg-brand-strong", "#1E2AFF");
        const neutralPrimaryColor = cssVar("--color-neutral-primary", "#FFFFFF");

        const options = {
            series: [52.8, 26.8, 20.4],
            labels: ["Direct", "Organic search", "Referrals"],
            colors: [brandColor, brandSecondaryColor, brandTertiaryColor],
            chart: {
                type: "pie",
                height: 420,
                width: "100%",
                fontFamily: "Inter, sans-serif"
            },
            stroke: {
                colors: [neutralPrimaryColor]
            },
            plotOptions: {
                pie: {
                    size: "100%",
                    dataLabels: {
                        offset: -25
                    }
                }
            },
            dataLabels: {
                enabled: true,
                style: {
                    fontFamily: "Inter, sans-serif"
                },
                formatter: (val) => `${val.toFixed(1)}%`
            },
            legend: {
                position: "bottom",
                fontFamily: "Inter, sans-serif"
            }
        };

        chart = new ApexCharts(chartEl, options);
        chart.render();
    });

    onDestroy(() => {
        chart?.destroy();
    });
</script>

<div class="max-w-sm w-full bg-neutral-primary-soft border border-default rounded-base shadow-xs p-4 md:p-6">

    <!-- Header -->
    <div class="flex justify-between items-start">
        <div>
            <h5 class="text-xl font-semibold text-heading">Website traffic</h5>
            <button class="text-sm text-fg-brand hover:underline">
                31 Nov - 31 Dec
            </button>
        </div>
    </div>

    <!-- Pie Chart -->
    <div class="py-6">
        <div bind:this={chartEl}></div>
    </div>

    <!-- Footer -->
    <div class="flex justify-between items-center border-t border-light pt-4">
        <button class="text-sm text-body hover:text-heading">
            Last 7 days
        </button>

        <a href="#" class="text-sm font-medium text-fg-brand">
            Traffic Report →
        </a>
    </div>

</div>
