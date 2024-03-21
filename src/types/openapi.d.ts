declare module 'astro-openapi:types' {
  import type {
    OpenAPIClient,
    Parameters,
    UnknownParamsObject,
    OperationResponse,
    AxiosRequestConfig,
  } from 'openapi-client-axios';
  declare namespace Components {
    namespace Schemas {
      export interface Error {
        code: number; // int32
        message: string;
      }
      export interface Lylic {
        id?: number; // int64
        quote: string;
        artist: string;
      }
    }
  }
  declare namespace Paths {
    namespace AddLylic {
      export type RequestBody = Components.Schemas.Lylic;
      namespace Responses {
        export type $200 = Components.Schemas.Lylic;
        export type Default = Components.Schemas.Error;
      }
    }
    namespace DeleteLylic {
      namespace Parameters {
        export type Id = number; // int64
      }
      export interface PathParameters {
        id: Parameters.Id /* int64 */;
      }
      namespace Responses {
        export interface $204 {}
        export type Default = Components.Schemas.Error;
      }
    }
    namespace FindPersonById {
      namespace Parameters {
        export type Id = number; // int64
      }
      export interface PathParameters {
        id: Parameters.Id /* int64 */;
      }
      namespace Responses {
        export type $200 = Components.Schemas.Lylic;
        export type Default = Components.Schemas.Error;
      }
    }
    namespace RandomLylic {
      namespace Responses {
        export type $200 = Components.Schemas.Lylic[];
        export type Default = Components.Schemas.Error;
      }
    }
  }

  export interface OperationMethods {
    /**
     * randomLylic - Return a random lylic.
     *
     */
    'randomLylic'(
      parameters?: Parameters<UnknownParamsObject> | null,
      data?: any,
      config?: AxiosRequestConfig
    ): OperationResponse<Paths.RandomLylic.Responses.$200>;
    /**
     * addLylic - Creates a new Lylic.
     */
    'addLylic'(
      parameters?: Parameters<UnknownParamsObject> | null,
      data?: Paths.AddLylic.RequestBody,
      config?: AxiosRequestConfig
    ): OperationResponse<Paths.AddLylic.Responses.$200>;
    /**
     * findPersonById - Returns a user based on a single ID, if the user does not have access to the person
     */
    'findPersonById'(
      parameters?: Parameters<Paths.FindPersonById.PathParameters> | null,
      data?: any,
      config?: AxiosRequestConfig
    ): OperationResponse<Paths.FindPersonById.Responses.$200>;
    /**
     * deleteLylic - deletes a single Lylic based on the ID supplied
     */
    'deleteLylic'(
      parameters?: Parameters<Paths.DeleteLylic.PathParameters> | null,
      data?: any,
      config?: AxiosRequestConfig
    ): OperationResponse<Paths.DeleteLylic.Responses.$204>;
  }

  export interface PathsDictionary {
    ['/lylics']: {
      /**
       * randomLylic - Return a random lylic.
       *
       */
      'get'(
        parameters?: Parameters<UnknownParamsObject> | null,
        data?: any,
        config?: AxiosRequestConfig
      ): OperationResponse<Paths.RandomLylic.Responses.$200>;
      /**
       * addLylic - Creates a new Lylic.
       */
      'post'(
        parameters?: Parameters<UnknownParamsObject> | null,
        data?: Paths.AddLylic.RequestBody,
        config?: AxiosRequestConfig
      ): OperationResponse<Paths.AddLylic.Responses.$200>;
    };
    ['/lylics/{id}']: {
      /**
       * findPersonById - Returns a user based on a single ID, if the user does not have access to the person
       */
      'get'(
        parameters?: Parameters<Paths.FindPersonById.PathParameters> | null,
        data?: any,
        config?: AxiosRequestConfig
      ): OperationResponse<Paths.FindPersonById.Responses.$200>;
      /**
       * deleteLylic - deletes a single Lylic based on the ID supplied
       */
      'delete'(
        parameters?: Parameters<Paths.DeleteLylic.PathParameters> | null,
        data?: any,
        config?: AxiosRequestConfig
      ): OperationResponse<Paths.DeleteLylic.Responses.$204>;
    };
  }

  export type Client = OpenAPIClient<OperationMethods, PathsDictionary>;
  declare global {
    namespace AstroOpenAPI {
      export { Client };
    }
  }
}
