import { createContext } from "react";
import { execute_command } from "./common";

export enum CommandScope {
    Application = "app",
    Kube = "kube",
    Helm = "helm",
    Kompose = "kompose",
    Artifacts = "artifacts",
}

export type CommandSpec<
    Scope extends CommandScope,
    Command extends string,
    Args extends object = Record<string, never>
> = {
    scope: Scope;
    command: Command;
} & Args;

export type CommandResult<
    Command extends CommandSpec<any, any, any>,
    Success = any
> = {
    command: Command;
} & ({ success: true; value: Success } | { success: false; error: string });

export type K8SGroup = {
    name: string;
};

export type ClusterConfig = {
    cluster_url: string;
    default_namespace: string;
};

export class ConnectionState {
    private clusterKey: string | null;
    private clusterConfig: ClusterConfig | null;
    private resourceCache: { [key: string]: any[] };

    private constructor(key: string | null, config: ClusterConfig | null) {
        this.clusterKey = key;
        this.clusterConfig = config;
        this.resourceCache = {};
    }

    public static active(key: string, config: ClusterConfig): ConnectionState {
        return new ConnectionState(key, config);
    }

    public static inactive(): ConnectionState {
        return new ConnectionState(null, null);
    }

    public get ready(): boolean {
        return this.clusterKey !== null;
    }

    public get name(): string | null {
        return this.clusterKey;
    }

    public get config(): ClusterConfig | null {
        return this.clusterConfig;
    }

    public async groups(): Promise<K8SGroup[]> {
        if (!this.ready) {
            return [];
        }

        const result = await execute_command<K8SGroup[]>(
            CommandScope.Kube,
            "supported_groups"
        );
        if (result.success) {
            return result.value;
        } else {
            return [];
        }
    }

    public async resources(group: K8SGroup): Promise<any[]> {
        if (!this.ready) {
            return [];
        }
        if (Object.keys(this.resourceCache).includes(group.name)) {
            return this.resourceCache[group.name];
        } else {
            const result = await execute_command<any[]>(
                CommandScope.Kube,
                "supported_resources",
                { group }
            );
            if (result.success) {
                return result.value;
            } else {
                return [];
            }
        }
    }
}

export type ClusterVersion = {
    build_date: string;
    compiler: string;
    git_commit: string;
    git_tree_state: string;
    git_version: string;
    go_version: string;
    major: string;
    minor: string;
    platform: string;
};

export type ClusterInfo = {
    config: ClusterConfig;
    connected: boolean;
    version: ClusterVersion | null;
};

export type ClusterMapping = {
    [key: string]: ClusterInfo;
};

export type ApiContextType = {
    clusters: ClusterMapping;
    connection: ConnectionState;
    setConnection: (connection: ConnectionState) => void;
    reload: () => Promise<ClusterMapping>;
    reloading: boolean;
};

export const ApiContext = createContext<ApiContextType>({
    clusters: {},
    connection: ConnectionState.inactive(),
    reload: async () => ({}),
    reloading: false,
    setConnection: () => {},
});
