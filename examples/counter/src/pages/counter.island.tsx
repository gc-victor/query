import { hydrate } from "preact";
import { signal } from "@preact/signals";
import { Counter } from "./counter";

const container = document.getElementById("counter");
const countText = document.getElementById("count")?.textContent ?? "0";
const count = signal(Number.parseInt(countText, 10));

hydrate(<Counter count={count} />, container as HTMLElement);
