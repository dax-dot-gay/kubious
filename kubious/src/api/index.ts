import { useContext } from "react";
import {
    ApiContext,
    ConnectionState,
    CommandResult,
    CommandScope,
    ClusterConfig,
    ClusterVersion,
    ApiContextType,
    ClusterMapping,
    ClusterInfo,
} from "./types";
import { ApiProvider } from "./ApiProvider";
import { execute_command } from "./common";

export { ApiProvider, execute_command, ConnectionState };
export type {
    CommandResult,
    CommandScope,
    ClusterConfig,
    ClusterVersion,
    ClusterInfo,
    ClusterMapping,
};

export function useApi(): ApiContextType {
    return useContext(ApiContext);
}

export function useConnection(): ConnectionState {
    return useApi().connection;
}

export function useClusters(): ClusterMapping {
    return useApi().clusters;
}

export function useCluster(key: string): null | ClusterInfo {
    const clusters = useClusters();
    return clusters[key] ?? null;
}
