<div class="skeleton"></div>

<style>
  .skeleton {
    width: 200px;
    height: 16px;
    background: linear-gradient(90deg, #eee 25%, #f5f5f5 37%, #eee 63%);
    background-size: 400% 100%;
    animation: shimmer 1.4s infinite;
  }

  @keyframes shimmer {
    0% {
      background-position: 100% 0;
    }
    100% {
      background-position: -100% 0;
    }
  }
</style>
