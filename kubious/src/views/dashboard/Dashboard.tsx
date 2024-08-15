import {
    ActionIcon,
    Box,
    ColorSwatch,
    Group,
    Stack,
    Text,
    Tooltip,
    useMantineColorScheme,
    useMantineTheme,
} from "@mantine/core";
import { mdiShipWheel } from "@mdi/js";
import Icon from "@mdi/react";
import { useTranslation } from "react-i18next";
import { DataTable } from "mantine-datatable";
import {
    ApplicationMixin,
    useApi,
    useClusters,
    useConnection,
    useReload,
} from "../../api";
import { useMemo } from "react";
import {
    IconPlug,
    IconPlugOff,
    IconPencil,
    IconTrashFilled,
    IconWifi,
    IconRefresh,
} from "@tabler/icons-react";

function ClusterTable() {
    const clusters = useClusters();
    const connection = useConnection();
    const { reloading, reload } = useReload();
    const { t } = useTranslation();
    const theme = useMantineTheme();
    const scheme = useMantineColorScheme();
    const { methods } = useApi(ApplicationMixin);

    const clusterData = useMemo(
        () =>
            Object.entries(clusters).map(([key, cluster]) => ({
                name: key,
                url: cluster.config.cluster_url,
                accessible: cluster.connected,
                connected: key === connection.name && cluster.connected,
                version: cluster.version
                    ? `${cluster.version.major}.${cluster.version.minor}`
                    : "-",
            })),
        [clusters, connection]
    );

    return (
        <Box className="dashboard-section clusters" p="sm">
            <Stack gap="sm">
                <Group gap="sm" justify="space-between">
                    <Group gap="sm">
                        <Icon path={mdiShipWheel} size="28px" />
                        <Text size="xl">
                            {t("views.dashboard.clusters.title")}
                        </Text>
                    </Group>
                    <ActionIcon
                        size="lg"
                        radius="xl"
                        variant="transparent"
                        color="gray"
                        disabled={reloading}
                        onClick={() => reload()}
                    >
                        <IconRefresh size={24} />
                    </ActionIcon>
                </Group>
                <DataTable
                    fetching={reloading}
                    records={reloading ? [] : clusterData}
                    withColumnBorders
                    withTableBorder
                    borderRadius="sm"
                    minHeight={
                        reloading || clusterData.length === 0
                            ? "150px"
                            : undefined
                    }
                    idAccessor="name"
                    columns={[
                        {
                            accessor: "accessible",
                            title: (
                                <IconWifi
                                    size={20}
                                    style={{ transform: "translate(1px, 3px)" }}
                                />
                            ),
                            render(record, _) {
                                return record.accessible ? (
                                    <ColorSwatch
                                        color={theme.colors.green[6]}
                                        size={20}
                                    />
                                ) : (
                                    <ColorSwatch
                                        color={theme.colors.red[6]}
                                        size={20}
                                    />
                                );
                            },
                        },
                        {
                            accessor: "connected",
                            title: (
                                <IconPlug
                                    size={20}
                                    style={{ transform: "translate(1px, 3px)" }}
                                />
                            ),
                            render(record, _) {
                                return record.connected ? (
                                    <ColorSwatch
                                        color={theme.colors.green[6]}
                                        size={20}
                                    />
                                ) : (
                                    <ColorSwatch
                                        color={theme.colors.red[6]}
                                        size={20}
                                    />
                                );
                            },
                        },
                        {
                            accessor: "name",
                            title: t("views.dashboard.clusters.header.name"),
                            width: "100%",
                        },
                        {
                            accessor: "url",
                            title: t("views.dashboard.clusters.header.url"),
                        },
                        {
                            accessor: "version",
                            title: t("views.dashboard.clusters.header.version"),
                            textAlign: "center",
                        },
                        {
                            accessor: "actions",
                            title: t("views.dashboard.clusters.header.actions"),
                            textAlign: "center",
                            render: (record) => (
                                <Group gap="xs" wrap="nowrap">
                                    {connection.name === record.name ? (
                                        <Tooltip
                                            label={t(
                                                "views.dashboard.clusters.action.disconnect"
                                            )}
                                            color={
                                                scheme.colorScheme === "dark"
                                                    ? "dark"
                                                    : "light"
                                            }
                                        >
                                            <ActionIcon
                                                size="md"
                                                variant="light"
                                                disabled={!record.accessible}
                                                color="red"
                                                onClick={() =>
                                                    methods.appSetCurrentConfig(
                                                        null
                                                    )
                                                }
                                            >
                                                <IconPlugOff size={16} />
                                            </ActionIcon>
                                        </Tooltip>
                                    ) : (
                                        <Tooltip
                                            label={t(
                                                "views.dashboard.clusters.action.connect"
                                            )}
                                            color={
                                                scheme.colorScheme === "dark"
                                                    ? "dark"
                                                    : "light"
                                            }
                                        >
                                            <ActionIcon
                                                size="md"
                                                variant="light"
                                                disabled={!record.accessible}
                                                color="green"
                                                onClick={() =>
                                                    methods.appSetCurrentConfig(
                                                        record.name
                                                    )
                                                }
                                            >
                                                <IconPlug size={16} />
                                            </ActionIcon>
                                        </Tooltip>
                                    )}
                                    <Tooltip
                                        label={t(
                                            "views.dashboard.clusters.action.edit"
                                        )}
                                        color={
                                            scheme.colorScheme === "dark"
                                                ? "dark"
                                                : "light"
                                        }
                                    >
                                        <ActionIcon
                                            size="md"
                                            variant="light"
                                            disabled={record.name === "default"}
                                            color="orange"
                                        >
                                            <IconPencil size="16" />
                                        </ActionIcon>
                                    </Tooltip>
                                    <Tooltip
                                        label={t(
                                            "views.dashboard.clusters.action.delete"
                                        )}
                                        color={
                                            scheme.colorScheme === "dark"
                                                ? "dark"
                                                : "light"
                                        }
                                    >
                                        <ActionIcon
                                            size="md"
                                            variant="light"
                                            disabled={record.name === "default"}
                                            color="red"
                                        >
                                            <IconTrashFilled size="16" />
                                        </ActionIcon>
                                    </Tooltip>
                                </Group>
                            ),
                        },
                    ]}
                />
            </Stack>
        </Box>
    );
}

export function Dashboard() {
    return (
        <Stack gap="sm">
            <ClusterTable />
        </Stack>
    );
}
