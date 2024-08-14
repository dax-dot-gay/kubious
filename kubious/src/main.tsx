import ReactDOM from "react-dom/client";
import { App } from "./App";
import "@mantine/core/styles.css";
import "@mantine/code-highlight/styles.css";
import "@mantine/charts/styles.css";
import "@mantine/notifications/styles.css";
import "@mantine/spotlight/styles.css";
import "mantine-contextmenu/styles.css";
import "mantine-datatable/styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <App />
);
