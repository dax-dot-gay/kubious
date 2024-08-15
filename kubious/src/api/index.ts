import { useContext, useMemo } from "react";
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
import { ApiMixin, BaseApi } from "./methods/base";
import { UnionToIntersection, ValuesType } from "utility-types";
import { useCustomCompareMemo } from "use-custom-compare";
import { difference } from "lodash";

import { ApplicationMixin } from "./methods/app";
import { ArtifactsMixin } from "./methods/artifacts";
import { HelmMixin } from "./methods/helm";
import { KomposeMixin } from "./methods/kompose";
import { KubeMixin } from "./methods/kube";

export {
    ApiProvider,
    execute_command,
    ConnectionState,
    ApplicationMixin,
    ArtifactsMixin,
    HelmMixin,
    KomposeMixin,
    KubeMixin,
};
export type {
    CommandResult,
    CommandScope,
    ClusterConfig,
    ClusterVersion,
    ClusterInfo,
    ClusterMapping,
};

export function useApiContext(): ApiContextType {
    return useContext(ApiContext);
}

export function useConnection(): ConnectionState {
    return useApiContext().connection;
}

export function useClusters(): ClusterMapping {
    return useApiContext().clusters;
}

export function useCluster(key: string): null | ClusterInfo {
    const clusters = useClusters();
    return clusters[key] ?? null;
}

export function useApi<TMixins extends ApiMixin<any, any>[]>(
    ...mixins: TMixins
): {
    connected: boolean;
    current: ClusterConfig | null;
    methods: typeof BaseApi &
        UnionToIntersection<ReturnType<ValuesType<TMixins>>["prototype"]>;
} {
    const api = useApiContext();
    const [connected, current] = useMemo(() => {
        return [api.connection.ready, api.connection.config];
    }, [api.connection.ready, api.connection.config?.cluster_url]);

    const methods = useCustomCompareMemo(
        () => {
            return new (mixins.reduce((prev, cur) => cur(prev), BaseApi))(api);
        },
        [api, mixins],
        ([prevApi, prevMixins], [nextApi, nextMixins]) => {
            if (prevApi.connection.ready !== nextApi.connection.ready) {
                return false;
            }

            if (prevApi.connection.name !== nextApi.connection.name) {
                return false;
            }

            if (
                JSON.stringify(prevApi.clusters) !==
                JSON.stringify(nextApi.clusters)
            ) {
                return false;
            }

            if (
                JSON.stringify(prevApi.connection.config) !==
                JSON.stringify(nextApi.connection.config)
            ) {
                return false;
            }

            if (
                difference(
                    prevMixins.map((v) => v.name),
                    nextMixins.map((v) => v.name)
                ).length > 0
            ) {
                return false;
            }

            return true;
        }
    );

    return {
        connected,
        current,
        methods: methods as any,
    };
}

export function useReload(): {
    reloading: boolean;
    reload: () => Promise<ClusterMapping>;
} {
    const api = useApiContext();
    return { reloading: api.reloading, reload: api.reload };
}
