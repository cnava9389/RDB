import { debounce } from ".";

export function handleOutsideClick(fn: () => void) {
  return (node: Node) => {
    const handleClick = (event: MouseEvent) => {
      if (!node.contains(event.target as Node)) {
        fn();
      }
    };

    document.addEventListener("click", handleClick, true);

    return {
      destroy() {
        document.removeEventListener("click", handleClick, true);
      },
    };
  };
}

export function handleBounce(node: Node & { classList: DOMTokenList }) {
  let timeout: NodeJS.Timeout;
  let className = "animate-bounce";
  node.classList.add("cursor-pointer");
  const handleClick = debounce(() => {
    node.classList.add(className);
    clearTimeout(timeout);
    timeout = setTimeout(() => {
      node.classList.remove(className);
    }, 200);
  }, 200) as EventListenerOrEventListenerObject;

  node.addEventListener("click", handleClick);

  return {
    destroy() {
      node.removeEventListener("click", handleClick);
    },
  };
}
