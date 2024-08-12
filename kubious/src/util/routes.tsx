import { createBrowserRouter } from "react-router-dom";
import { Layout } from "../views/layout/Layout";

export const appRouter = createBrowserRouter([
    {
        path: "/",
        element: <Layout />,
    },
]);
