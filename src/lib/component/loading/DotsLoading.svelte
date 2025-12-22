<div class="dots">
  <span></span><span></span><span></span>
</div>

<style>
  .dots span {
    width: 8px;
    height: 8px;
    background: #3b82f6;
    border-radius: 50%;
    display: inline-block;
    animation: bounce 1.4s infinite both;
  }

  .dots span:nth-child(1) {
    animation-delay: -0.32s;
  }
  .dots span:nth-child(2) {
    animation-delay: -0.16s;
  }

  @keyframes bounce {
    0%,
    80%,
    100% {
      transform: scale(0);
    }
    40% {
      transform: scale(1);
    }
  }
</style>
