open Graphql_result

(** GraphQL schema signature *)
module type Schema = sig
  type +'a io

  (** {3 Base types } *)

  type 'ctx schema

  type ('ctx, 'src) field

  type ('ctx, 'src) typ

  type 'a enum_value

  (** {3 Constructors } *)

  val schema :
       ?mutation_name:string
    -> ?mutations:('ctx, unit) field list
    -> ?query_name:string
    -> ('ctx, unit) field list
    -> 'ctx schema

  type deprecated = NotDeprecated | Deprecated of string option

  val enum_value :
       ?doc:string
    -> ?deprecated:deprecated
    -> string
    -> value:'a
    -> 'a enum_value

  val obj :
       ?doc:string
    -> string
    -> fields:(('ctx, 'src option) typ -> ('ctx, 'src) field list)
    -> ('ctx, 'src option) typ

  module Arg : sig
    type _ arg

    type _ arg_typ

    type (_, _) arg_list =
      | Nil : ('a, 'a) arg_list
      | Cons: 'a arg * ('b, 'c) arg_list -> ('b, 'a -> 'c) arg_list

    val arg : ?doc:string -> string -> typ:'a arg_typ -> 'a arg

    val arg' :
      ?doc:string -> string -> typ:'a option arg_typ -> default:'a -> 'a arg

    val scalar :
         ?doc:string
      -> string
      -> coerce:(Js.Json.t -> ('a, string) result)
      -> 'a option arg_typ

    val enum :
      ?doc:string -> string -> values:'a enum_value list -> 'a option arg_typ

    val obj :
         ?doc:string
      -> string
      -> fields:('a, 'b) arg_list
      -> coerce:'b
      -> 'a option arg_typ

    (* Argument constructors *)

    val int : int option arg_typ

    val string : string option arg_typ

    val bool : bool option arg_typ

    val float : float option arg_typ

    val guid : string option arg_typ

    val list : 'a arg_typ -> 'a list option arg_typ

    val non_null : 'a option arg_typ -> 'a arg_typ
  end

  val field :
       ?doc:string
    -> ?deprecated:deprecated
    -> string
    -> typ:('ctx, 'a) typ
    -> args:('a, 'b) Arg.arg_list
    -> resolve:('ctx -> 'src -> 'b)
    -> ('ctx, 'src) field

  val io_field :
       ?doc:string
    -> ?deprecated:deprecated
    -> string
    -> typ:('ctx, 'a) typ
    -> args:(('a, string) result io, 'b) Arg.arg_list
    -> resolve:('ctx -> 'src -> 'b)
    -> ('ctx, 'src) field

  val enum :
    ?doc:string -> string -> values:'a enum_value list -> ('ctx, 'a option) typ

  val scalar :
    ?doc:string -> string -> coerce:('a -> Js.Json.t) -> ('ctx, 'a option) typ

  val list : ('ctx, 'src) typ -> ('ctx, 'src list option) typ

  val non_null : ('ctx, 'src option) typ -> ('ctx, 'src) typ

  (** {3 Built-in scalars} *)

  val int : ('ctx, int option) typ

  val string : ('ctx, string option) typ

  val guid : ('ctx, string option) typ

  val bool : ('ctx, bool option) typ

  val float : ('ctx, float option) typ
  (* type variables = (string * Js.Json.t) list *)
  (*val execute : *)
  (*     'ctx schema *)
  (*  -> 'ctx *)
  (*  -> ?variables:variables *)
  (*  -> ?operation_name:string *)
  (*  -> Graphql_parser.document *)
  (*  -> (Yojson.Basic.json, Yojson.Basic.json) result io *)
  (*(1** [execute schema ctx variables doc] evaluates the [doc] against [schema] *)
  (*    with the given context [ctx] and [variables]. *1) *)
end
