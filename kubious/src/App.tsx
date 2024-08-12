import "./styles/index.scss";
import { createTheme, MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import {} from "@mantine/spotlight";
import { ModalsProvider } from "@mantine/modals";
import { RouterProvider } from "react-router-dom";
import { appRouter } from "./util/routes";
import { LocalizationProvider } from "./util/localization";

export function App() {
    return (
        <LocalizationProvider>
            <MantineProvider
                defaultColorScheme="dark"
                theme={createTheme({
                    colors: {
                        primary: [
                            "#f2f0ff",
                            "#e0dff2",
                            "#bfbdde",
                            "#9b98ca",
                            "#7d79ba",
                            "#6a65b0",
                            "#605bac",
                            "#504c97",
                            "#464388",
                            "#3b3979",
                        ],
                    },
                    primaryColor: "primary",
                    primaryShade: 5,
                })}
            >
                <Notifications />
                <ModalsProvider>
                    <RouterProvider router={appRouter} />
                </ModalsProvider>
            </MantineProvider>
        </LocalizationProvider>
    );
}
