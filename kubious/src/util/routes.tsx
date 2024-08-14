import { createBrowserRouter } from "react-router-dom";
import { Layout } from "../views/layout/Layout";
import { Dashboard } from "../views/dashboard/Dashboard";

export const appRouter = createBrowserRouter([
    {
        path: "/",
        element: <Layout />,
        children: [
            {
                path: "/",
                element: <Dashboard />,
            },
        ],
    },
]);
