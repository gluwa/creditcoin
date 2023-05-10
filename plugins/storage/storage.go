package storage

import (
	"github.com/itering/substrate-api-rpc/websocket"
	"github.com/shopspring/decimal"
)

type Dao interface {
	DB
	// Find spec metadata raw
	SpecialMetadata(int) string

	// Substrate websocket rpc pool
	RPCPool() *websocket.PoolConn

	// Plugin set prefix
	SetPrefix(string)
}

type Option struct {
	PluginPrefix string
	PageSize     int
	Page         int
	Order        string
}

// DB interface
// Every query can be found here https://gorm.io/docs/
type DB interface {
	// Can query database all tables data
	// Query ** no prefix ** table default, option PluginPrefix can specify other plugin model
	FindBy(record interface{}, query interface{}, option *Option) (int, bool)

	// Only can exec plugin relate tables
	// Migration
	AutoMigration(model interface{}) error
	// Add column Index
	AddIndex(model interface{}, indexName string, columns ...string) error
	// Add column unique index
	AddUniqueIndex(model interface{}, indexName string, columns ...string) error

	// Create one record
	Create(record interface{}) error
	// Update one or more column
	Update(model interface{}, query interface{}, attr map[string]interface{}) error
	// Delete one or more record
	Delete(model interface{}, query interface{}) error
}

type Block struct {
	BlockNum       int    `json:"block_num"`
	BlockTimestamp int    `json:"block_timestamp"`
	Hash           string `json:"hash"`
	SpecVersion    int    `json:"spec_version"`
	Validator      string `json:"validator"`
	Finalized      bool   `json:"finalized"`
}

type Extrinsic struct {
	ExtrinsicIndex     string          `json:"extrinsic_index" `
	CallCode           string          `json:"call_code"`
	CallModuleFunction string          `json:"call_module_function" `
	CallModule         string          `json:"call_module"`
	Params             []byte          `json:"params"`
	AccountId          string          `json:"account_id"`
	Signature          string          `json:"signature"`
	Nonce              int             `json:"nonce"`
	Era                string          `json:"era"`
	ExtrinsicHash      string          `json:"extrinsic_hash"`
	Success            bool            `json:"success"`
	Fee                decimal.Decimal `json:"fee"`
}

type Event struct {
	BlockNum      int    `json:"block_num"`
	ExtrinsicIdx  int    `json:"extrinsic_idx"`
	ModuleId      string `json:"module_id"`
	EventId       string `json:"event_id"`
	Params        []byte `json:"params"`
	ExtrinsicHash string `json:"extrinsic_hash"`
	EventIdx      int    `json:"event_idx"`
}

type ExtrinsicParam struct {
	Name  string      `json:"name"`
	Type  string      `json:"type"`
	Value interface{} `json:"value"`
}

type EventParam struct {
	Type  string      `json:"type"`
	Value interface{} `json:"value"`
}
