import { ReactNode, useCallback, useEffect, useState } from "react";
import {
    ApiContext,
    ClusterConfig,
    ClusterMapping,
    CommandScope,
    ConnectionState,
} from "./types";
import { execute_command } from "./common";

export function ApiProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    const [clusters, setClusters] = useState<ClusterMapping>({});
    const [connection, setConnection] = useState<ConnectionState>(
        ConnectionState.inactive()
    );
    const [reloading, setReloading] = useState(false);

    const reload = useCallback(async () => {
        setReloading(true);
        const config_statuses = await execute_command<ClusterMapping>(
            CommandScope.Application,
            "check_configs"
        );
        const current = await execute_command<[string, ClusterConfig] | null>(
            CommandScope.Application,
            "get_current_config"
        );

        if (config_statuses.success) {
            setClusters(config_statuses.value);
        } else {
            setClusters({});
        }

        if (current.success) {
            if (current.value) {
                setConnection(
                    ConnectionState.active(current.value[0], current.value[1])
                );
            } else {
                setConnection(ConnectionState.inactive());
            }
        } else {
            setConnection(ConnectionState.inactive());
        }
        setReloading(false);
        return config_statuses.success ? config_statuses.value : {};
    }, [setClusters, setConnection, setReloading]);

    useEffect(() => {
        reload();
    }, []);

    return (
        <ApiContext.Provider
            value={{ clusters, connection, reload, reloading, setConnection }}
        >
            {children}
        </ApiContext.Provider>
    );
}
