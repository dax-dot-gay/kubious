import { Box, Button, Divider, Group, Paper, Stack, Text } from "@mantine/core";
import { Outlet } from "react-router-dom";
import { Icon } from "@mdi/react";
import { mdiCube } from "@mdi/js";
import { useTranslation } from "react-i18next";
import { execute_command } from "../../api/common";
import { CommandScope } from "../../api/types";

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
                        <Divider />
                        <Button
                            onClick={() =>
                                execute_command(
                                    CommandScope.Kube,
                                    "supported_groups"
                                ).then(console.log)
                            }
                        >
                            TEST
                        </Button>
                    </Stack>
                </Paper>
                <Box className="layout-content">
                    <Outlet />
                </Box>
            </Group>
        </Box>
    );
}
