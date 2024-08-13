import { Box, Button, Divider, Group, Paper, Stack, Text } from "@mantine/core";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { Icon } from "@mdi/react";
import { mdiCube, mdiViewDashboard } from "@mdi/js";
import { useTranslation } from "react-i18next";

function NavButton({
    icon,
    label,
    href,
}: {
    icon: string;
    label: string;
    href: string;
}) {
    const nav = useNavigate();
    const location = useLocation();
    return (
        <Button
            leftSection={<Icon size="20px" path={icon} />}
            justify="space-between"
            onClick={() => nav(href)}
            variant={location.pathname === href ? "filled" : "light"}
        >
            {label}
        </Button>
    );
}

export function Layout() {
    const { t } = useTranslation();
    return (
        <Box className="app-root">
            <Group gap="sm" wrap="nowrap" className="layout-group" p="xs">
                <Paper
                    className="paper-light layout-nav"
                    p="sm"
                    radius="sm"
                    shadow="sm"
                >
                    <Stack gap="sm" className="nav-stack">
                        <Group gap="sm" justify="space-between" pr="xs">
                            <Icon size="32px" path={mdiCube} />
                            <Text size="lg">{t("common.appName")}</Text>
                        </Group>
                        <NavButton
                            icon={mdiViewDashboard}
                            label={t("views.dashboard.nav")}
                            href="/"
                        />
                        <Divider />
                    </Stack>
                </Paper>
                <Box className="layout-content">
                    <Outlet />
                </Box>
            </Group>
        </Box>
    );
}
